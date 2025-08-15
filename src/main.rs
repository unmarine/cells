use std::time::Instant;
use crate::automaton::Automaton;
use crate::neighbours::{moore};
use crate::State::{Dead, Live};

mod automaton;
mod neighbours;
mod grid;
mod condition;

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Live = 0,
    Dead = 1
}

fn main() {
    let (width, height) = (1280, 720);
    let mut automaton: Automaton = Automaton::new(height, width);

    // Conway's Game of Life
    // automaton.add_condition(0, Live, Live, Dead);
    // automaton.add_condition(1, Live, Live, Dead);
    // automaton.add_condition(2, Live, Live, Live);
    // automaton.add_condition(3, Live, Live, Live);
    // automaton.add_condition_range(4, 7, Live, Live, Dead);
    // automaton.add_condition(3, Live, Dead, Live);

    automaton.add_condition(3, Live, Dead, Live);
    automaton.add_condition(6, Live, Dead, Live);
    automaton.add_condition(7, Live, Dead, Live);
    automaton.add_condition(8, Live, Dead, Live);
    automaton.add_condition(3, Live, Live, Live);
    automaton.add_condition(4, Live, Live, Live);
    automaton.add_condition(6, Live, Live, Live);
    automaton.add_condition(7, Live, Live, Live);
    automaton.add_condition(8, Live, Live, Live);

    automaton.grid.randomize();
    let mut counter = 0;
    let limit = 1000;
    let start_time = Instant::now();
    loop {
        if counter >= limit {
            break;
        }
        counter += 1;
        automaton.update(moore);
        println!("{}/{}", counter, limit);
        automaton.produce_image(1, format!("./images/{}.png", counter).as_str() ).expect("Failed drawing");
    }
    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);
}
