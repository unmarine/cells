use rand::Rng;
use crate::State;
use crate::State::{Dead, Live};

pub struct Grid {
    pub height: usize,
    pub width: usize,
    grid: Vec<Vec<State>>
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        let mut grid: Vec<Vec<State>> = vec![vec![Dead; width]; height];
        for i in 0..height {
            for j in 0..width {
                grid[i][j] = Dead;
            }
        }
        Grid {
            height, width, grid
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::rng();
        for i in 0..self.height {
            for j in 0..self.width {
                self.grid[i][j] = match rng.random_range(0..2) {
                    0 => Dead,
                    _ => Live,
                }
            }
        }
    }

    pub fn return_at_pos(&self, x: usize, y: usize) -> State {
        self.grid[y][x]
    }

    pub fn set_at_pos(&mut self, x: usize, y: usize, to_set: &State) {
        self.grid[y][x] = *to_set;
    }
}
