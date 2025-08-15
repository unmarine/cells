use std::time::Instant;
use crate::automaton::Automaton;
use crate::neighbours::{moore};
use crate::State::{Dead, Dying, Live};

mod automaton;
mod neighbours;
mod grid;
mod condition;

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Live,
    Dead,
    Dying
}

fn main() {
    let (width, height) = (1280, 720);
    let mut automaton: Automaton = Automaton::new(height, width);

    // Conway's Game of Life
    automaton.add_condition(0, Live, Live, Dead);
    automaton.add_condition(1, Live, Live, Dead);
    automaton.add_condition(2, Live, Live, Live);
    automaton.add_condition(3, Live, Live, Live);
    automaton.add_condition_range(4, 7, Live, Live, Dead);
    automaton.add_condition(3, Live, Dead, Live);

    // Day and Night
    // automaton.add_condition(3, Live, Dead, Live);
    // automaton.add_condition(6, Live, Dead, Live);
    // automaton.add_condition(7, Live, Dead, Live);
    // automaton.add_condition(8, Live, Dead, Live);
    // automaton.add_condition(3, Live, Live, Live);
    // automaton.add_condition(4, Live, Live, Live);
    // automaton.add_condition(6, Live, Live, Live);
    // automaton.add_condition(7, Live, Live, Live);
    // automaton.add_condition(8, Live, Live, Live);

    //
    // automaton.add_condition(-1, Live, Live, Dying);
    // automaton.add_condition(2, Live, Dead, Live);
    // automaton.add_condition(0, Live, Dead, Dead);
    // automaton.add_condition(1, Live, Dead, Dead);
    // automaton.add_condition(-1, Live, Dying, Dead);
    // automaton.add_condition_range(3, 7, Live, Dead, Dead);

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
        automaton.produce_image(4, format!("./images/{}.png", counter).as_str() ).expect("Failed drawing");
    }
    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);
}
