use rand::Rng;
use crate::environment::environment::{State, Action, Reward, Environment};
use crate::environment::tools;

pub struct LineWorld {
    pub agent_position: i64,
    pub all_position: Vec<i64>,
    pub terminal_position: Vec<i64>,
    pub all_actions: Vec<i64>,
    pub rewards : Vec<Vec<Vec<f64>>>,
    pub probabilities: Vec<Vec<Vec<f64>>>
}

impl LineWorld {
    pub fn new(len: i64, is_rand: bool, pos: i64) -> Box<LineWorld> {
        let mut env = Box::new(LineWorld {
            agent_position: if !is_rand {
                pos
            } else {
                let mut rng = rand::thread_rng();
                rng.gen_range(1..len)
            },
            all_position: (0..len).collect(),
            terminal_position:vec![0, len-1],
            // 0 : Stand / 1 : Left / 2 : Right
            all_actions: vec![0, 1, 2],
            rewards: vec![vec![vec![0.0; len as usize]; 3]; len as usize],
            probabilities: vec![vec![vec![0.0; len as usize]; 3]; len as usize]
        });
        env.generate_rewards();
        env.generate_probabilities();
        env
    }

    pub fn is_game_over(&self) -> bool {
        self.terminal_position.contains(&self.agent_position)
    }

    pub fn reset(&mut self, is_rand: bool, pos: i64) -> State {
        self.agent_position = if !is_rand {
            pos
        } else {
            let mut rng = rand::thread_rng();
            rng.gen_range(1..self.all_position[self.all_position.len() - 1])
        };
        self.agent_position
    }

    fn generate_rewards(&mut self) {
        let num_positions = self.all_position.len();
        let num_actions = self.all_actions.len();

        let current_position = self.agent_position;
        for position_index in 0..num_positions {
            self.agent_position = position_index as i64;
            for action_index in 0..num_actions {
                let action = self.all_actions[action_index];

                if self.available_actions().contains(&action) { self.step(action); }
                let next_state = if self.state_id() == 0 { 0 } else { self.state_id()};
                let reward = self.score();

                self.rewards[position_index][action_index][next_state as usize] = reward;

                self.agent_position = position_index as i64;
            }
        }
        self.agent_position = current_position;
    }

    fn generate_probabilities(&mut self){
        let num_positions = self.all_position.len();
        let num_actions = self.all_actions.len();
        let begin_position = self.agent_position;
        for position_index in 0..num_positions {
            let current_position = position_index as i64; // Positions de 1 à len
            for action_index in 0..num_actions {
                let action = self.all_actions[action_index];
                let available_act = self.available_actions();

                if available_act.contains(&action) {
                    self.agent_position = current_position;
                    self.step(action);

                    let next_state = self.state_id() as usize; // Ajustement pour l'index 0-based
                    self.probabilities[position_index][action_index][next_state] = 1.0;

                    // Remettre l'agent à la position initiale pour le prochain essai
                    self.agent_position = current_position;
                }
            }
        }
        self.agent_position = begin_position;
    }
    fn get_display_array(&mut self) -> Vec<char>{
        let mut renderer: Vec<char>= Vec::new();
        for i in self.all_position[0]..self.all_position.len() as i64 {
            if self.agent_position == i { renderer.push('X') } else {renderer.push('_') }
        }
        let game: String = renderer.iter().collect();
        println!("{}", game);
        renderer
    }
}

impl Environment for LineWorld {
    fn reset(&mut self) -> State {
        let mut rng = rand::thread_rng();
        self.agent_position = rng.gen_range(0..self.all_position.len() as i64);
        self.agent_position
    }

    fn step(&mut self, action: Action) -> (State, Reward, bool) {
        match action {
            1 if self.agent_position > 0 => self.agent_position -= 1,
            2 if self.agent_position < self.all_position.len() as i64 => self.agent_position += 1,
            _ => {}
        }

        let reward = self.score();
        let done = self.is_game_over();
        (self.agent_position, reward, done)
    }

    fn available_actions(&self) -> Vec<Action> {
        let mut actions = vec![0];
        if self.agent_position > 0 { actions.push(1); }
        if self.agent_position < self.all_position.len() as i64 - 1  { actions.push(2); }
        actions
    }

    fn all_states(&self) -> Vec<State> {
        self.all_position.clone()
    }

    fn terminal_states(&self) -> Vec<State> { self.terminal_position.clone() }

    fn set_state(&mut self, state: State) {
        self.agent_position = state;
    }

    fn display(&self) {
        let game: String = self.all_position.iter().map(|&pos| if pos == self.agent_position { 'X' } else { '_' }).collect();
        println!("{}", game);
    }

    fn state_id(&self) -> State {
        self.agent_position
    }

    fn score(&self) -> Reward {
        tools::score(self.agent_position, &self.terminal_position)
    }

    fn is_game_over(&self) -> bool {
        self.terminal_position.contains(&self.agent_position)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_line_world() -> Box<LineWorld>{
        let env = LineWorld::new(4, false, 1);
        env
    }

    #[test]
    fn test_init() {
        let env = setup_line_world();
        assert_eq!(env.agent_position, 1);
        assert_eq!(env.all_position, vec![0, 1, 2, 3]);
        assert_eq!(env.terminal_position, vec![0, 3]);
    }
    #[test]
    fn test_is_game_over() {
        let mut env = setup_line_world();
        assert_eq!(env.is_game_over(), false);
        env.agent_position = 3;
        assert_eq!(env.is_game_over(), true);
    }
    #[test]
    fn test_state_id() {
        let mut env = setup_line_world();
        assert_eq!(env.state_id(), 1);
        env.agent_position = 3;
        assert_eq!(env.state_id(), 3);
    }
    #[test]
    fn test_step() {
        let mut env = setup_line_world();
        for _ in 0..2 {
            env.step(2);
        }
        assert_eq!(env.agent_position, 3);
    }
    #[test]
    fn test_score() {
        let mut env = setup_line_world();
        env.agent_position = 3;
        assert_eq!(env.score(), 1.0);
        env.agent_position = 0;
        assert_eq!(env.score(), -1.0);
    }
    #[test]
    fn test_display() {
        let mut env = setup_line_world();
        let array = env.get_display_array();
        assert_eq!(array, vec!['_', 'X', '_', '_']);
    }
    #[test]
    fn test_reset() {
        let mut env = setup_line_world();
        env.agent_position = 3;
        env.reset(false, 1);
        assert_eq!(env.agent_position, 1);
    }
}
