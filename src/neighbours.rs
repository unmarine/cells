use crate::grid::Grid;
use crate::State;

#[derive(Clone)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize
}



pub fn moore (grid: &Grid, compared: &State, coordinate: &Coordinate) -> usize {
    let mut count = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let x = ((coordinate.x as i32) + i + (grid.width as i32)) % grid.width as i32;
            let y = ((coordinate.y as i32) + j + (grid.height as i32)) % grid.height as i32;

            if x < 0 || x >= grid.width as i32 || y < 0 || y >= grid.height as i32 {
                continue;
            }

            if grid.return_at_pos(x as usize, y as usize) == *compared {
                count += 1;
            }
        }
    }

    count
}