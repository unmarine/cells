use image::{ImageBuffer, Rgb};
use crate::condition::Condition;
use crate::grid::Grid;
use crate::neighbours::{Coordinate};
use crate::State;
use crate::State::{Dead, Live};


pub struct Automaton{
    pub grid: Grid,
    conditions: Vec<Condition>
}

impl Automaton {
    pub fn new(height: usize, width: usize) -> Self {
        Automaton {
            grid: Grid::new(height, width),
            conditions: vec![]
        }
    }

    pub fn add_condition(&mut self, amount_to_switch: isize, condition_state: State, initial_state: State, result_state: State) {
        self.conditions.push(Condition::new(amount_to_switch, initial_state, result_state, condition_state));
    }

    pub fn add_condition_range(&mut self, from: usize, until: usize, condition_state: State, initial_state: State, result_state: State) {
        for i in from..=until {
            self.add_condition(i as isize, condition_state, initial_state, result_state);
        }
    }

    pub fn update(&mut self, neighbourhood: fn(grid: &Grid, compared: &State, coordinate: &Coordinate) -> usize) {
        let mut updated_grid: Grid = Grid::new(self.grid.height, self.grid.width);

        for i in 0..self.grid.height {
            for j in 0..self.grid.width {
                let current_state = self.grid.return_at_pos(j, i);

                for condition in &self.conditions {
                    if current_state != condition.initial_state {
                        continue;
                    }

                    let coordinate = Coordinate {
                        y: i, x: j
                    };

                    if condition.amount_to_switch != -1 && neighbourhood(&self.grid, &condition.condition_state, &coordinate) == condition.amount_to_switch as usize{
                        updated_grid.set_at_pos(j, i, &condition.result_state);
                    } else if condition.amount_to_switch == -1 {
                        updated_grid.set_at_pos(j, i, &condition.result_state);
                    }
                }

            }
        }

        self.grid = updated_grid;
    }

    pub fn produce_image(&self, scale: usize, filename: &str) -> Result<(), image::ImageError> {
        let mut img: ImageBuffer<Rgb<u8>, Vec<u8>>= ImageBuffer::new(self.grid.width as u32, self.grid.height as u32);

        for x in 0..self.grid.width / scale {
            for y in 0..self.grid.height / scale {
                let state = self.grid.return_at_pos(x, y);
                let color = match state {
                    Live => Rgb([0, 0, 255]),
                    Dead => Rgb([0, 0, 0]),
                    State::Dying => Rgb([255, 255, 255])
                };

                for i in 0..scale {
                    for j in 0..scale {
                        let pixel_x = x * scale + i;
                        let pixel_y = y * scale + j;
                        img.put_pixel(pixel_x as u32, pixel_y as u32, color);
                    }
                }
            }
        }
        img.save(filename)
    }
}

