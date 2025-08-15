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

    automaton.add_condition(0, Live, Live, Dead);
    automaton.add_condition(1, Live, Live, Dead);
    automaton.add_condition(2, Live, Live, Live);
    automaton.add_condition(3, Live, Live, Live);
    automaton.add_condition_range(4, 8, Live, Live, Dead);
    automaton.add_condition(3, Live, Dead, Live);
    automaton.grid.randomize();

    let mut counter = 0;
    let start_time = Instant::now();
    loop {
        if counter == 1000 {
            break;
        }
        counter += 1;
        automaton.update(moore);
        println!("workin");
        automaton.produce_image(format!("./images/{}.png", counter).as_str() ).expect("Failed drawing");
    }
    let duration = start_time.elapsed();
    println!("Time taken: {:?}", duration);
}
