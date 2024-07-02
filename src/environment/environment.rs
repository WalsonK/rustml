extern crate rand;
use rand::Rng;

pub type State = usize;
pub type Action = usize;
pub type Reward = f32;

pub trait Environment {
    fn reset(&mut self) -> State;
    fn step(&mut self, action: Action) -> (State, Reward, bool);
    fn available_actions(&self) -> Vec<Action>;
    fn all_states(&self) -> Vec<State>;
    fn set_state(&mut self, statxe: State);
    fn display(&self);
    fn state_id(&self) -> State;
    fn score(&self) -> Reward;
    fn is_game_over(&self) -> bool;
}