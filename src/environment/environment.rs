extern crate rand;

pub type State = usize;
pub type Action = usize;
pub type Reward = f32;

pub trait Environment {
    fn reset(&mut self) -> State;
    fn step(&mut self, action: Action) -> (State, Reward, bool);
    fn available_actions(&self) -> Vec<Action>;
    fn all_states(&self) -> Vec<State>;
    fn terminal_states(&self) -> Vec<State>;
    fn set_state(&mut self, state: State);
    fn display(&self);
    fn state_id(&self) -> State;
    fn score(&self) -> Reward;
    fn is_game_over(&self) -> bool;
    fn all_action(&self) -> Vec<State>;
    fn is_forbidden(&self, state_or_action: usize) -> bool;
    fn transition_probability(&self, state: usize, action: usize, next_state: usize, reward: usize) -> f32;
    fn random_state(&mut self) ;
}