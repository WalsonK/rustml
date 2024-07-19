extern crate rand;
use rand::Rng;
use crate::environment::environment::{State, Action, Reward, Environment};
pub struct playable_line_world {
    agent_pos: usize,
    all_pos: Vec<usize>,
    go_pos: Vec<usize>,
}

impl playable_line_world {
    pub fn new(len: usize, is_rand: bool, pos: usize) -> Box<playable_line_world> {
        let agent_pos = if !is_rand {
            pos
        } else {
            let mut rng = rand::thread_rng();
            rng.gen_range(1..=len)
        };

        Box::new(playable_line_world {
            agent_pos,
            all_pos: (1..=len).collect(),
            go_pos: vec![1, len],
        })
    }

    pub fn is_game_over(&self) -> bool {
        self.go_pos.contains(&self.agent_pos)
    }
}

impl Environment for playable_line_world {
    fn reset(&mut self) -> State {
        self.agent_pos = rand::thread_rng().gen_range(1..=self.all_pos.len()).try_into().unwrap();
        self.agent_pos
    }

    fn step(&mut self, action: Action) -> (State, Reward, bool) {
        match action {
            1 if self.agent_pos > 1 => self.agent_pos -= 1,
            2 if self.agent_pos < self.all_pos.len() => self.agent_pos += 1,
            _ => {}
        }
        let reward = self.score();
        let done = self.is_game_over();
        (self.agent_pos, reward, done)
    }

    fn available_actions(&self) -> Vec<Action> {
        let mut actions = vec![0];
        if self.agent_pos > 1 { actions.push(1); }
        if self.agent_pos < self.all_pos.len() { actions.push(2); }
        actions
    }

    fn all_states(&self) -> Vec<State> {
        self.all_pos.clone()
    }

    fn set_state(&mut self, state: State) {
        self.agent_pos = state;
    }

    fn display(&self) {
        let game: String = self.all_pos.iter().map(|&pos| if pos == self.agent_pos { 'X' } else { '_' }).collect();
        println!("{}", game);
    }

    fn state_id(&self) -> State {
        self.agent_pos
    }

    fn score(&self) -> Reward {
        if self.agent_pos == self.go_pos[0] {
            -1.0
        } else if self.agent_pos == self.go_pos[1] {
            1.0
        } else {
            0.0
        }
    }

    fn is_game_over(&self) -> bool {
        self.go_pos.contains(&self.agent_pos)
    }
    fn all_action(&self) -> Vec<Action> {
        self.all_pos.iter().map(|&action| action as Action).collect()
    }

    fn is_forbidden(&self, state_or_action: usize) -> bool{
        false
    }
}
