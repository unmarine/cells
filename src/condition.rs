use crate::State;

pub struct Condition {
    pub amount_to_switch: isize,
    pub initial_state: State,
    pub result_state: State,
    pub condition_state: State
}

impl Condition {
    pub fn new(amount_to_switch: isize, initial_state: State, result_state: State, condition_state: State) -> Self {
        Condition {
            amount_to_switch, initial_state, result_state, condition_state
        }
    }
}