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

    pub fn display(&self) {
        for i in 0..self.grid.height {
            for j in 0..self.grid.width {
                let a = match self.grid.return_at_pos(j, i) {
                    Live => '#',
                    Dead => ' '
                };
                print!("{}", a);
            }
            println!();
        }
    }

    pub fn add_condition(&mut self, amount_to_switch: usize, condition_state: State, initial_state: State, result_state: State) {
        let condition = Condition {
            amount_to_switch, condition_state, initial_state, result_state
        };
        self.conditions.push(condition);
    }

    pub fn add_condition_range(&mut self, from: usize, until: usize, condition_state: State, initial_state: State, result_state: State) {
        for i in from..until {
            self.add_condition(i, condition_state, initial_state, result_state);
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

                    if neighbourhood(&self.grid, &condition.condition_state, &coordinate) == condition.amount_to_switch {
                        updated_grid.set_at_pos(j, i, &condition.result_state);
                    }
                }

            }
        }

        self.grid = updated_grid;
    }

    pub fn produce_image(&self, filename: &str) -> Result<(), image::ImageError> {
        let mut img: ImageBuffer<Rgb<u8>, Vec<u8>>= ImageBuffer::new(self.grid.width as u32, self.grid.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let state = self.grid.return_at_pos(x as usize, y as usize);
            let color = match state {
              Live => Rgb([255, 0, 0]),
              Dead => Rgb([0, 0, 0])
            };
            *pixel = color;
        }
        img.save(filename)
    }
}

