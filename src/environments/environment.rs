extern crate rand;
use rand::Rng;

pub type State = i64;
pub type Action = i64;
pub type Reward = f64;

pub trait Environment {
    fn reset(&mut self) -> State;
    fn step(&mut self, action: Action) -> (State, Reward, bool);
    fn available_actions(&self) -> Vec<Action>;
    fn all_states(&self) -> Vec<State>;
    fn set_state(&mut self, state: State);
    fn display(&self);
    fn state_id(&self) -> State;
    fn score(&self) -> Reward;
}
