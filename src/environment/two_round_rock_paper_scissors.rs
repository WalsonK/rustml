use rand::Rng;
use crate::environment::environment::{State, Action as ActionType, Reward, Environment};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action_game {
    Rock,
    Paper,
    Scissors,
}

impl Action_game {
    fn beats(&self, other: Action_game) -> i32 {
        match (self, other) {
            (Action_game::Rock, Action_game::Scissors) | (Action_game::Scissors, Action_game::Paper) | (Action_game::Paper, Action_game::Rock) => 1,
            (Action_game::Scissors, Action_game::Rock) | (Action_game::Paper, Action_game::Scissors) | (Action_game::Rock, Action_game::Paper) => -1,
            _ => 0,
        }
    }
}

pub struct RPSGame {
    agent_action: Option<Action_game>,
    adversary_action: Option<Action_game>,
    first_agent_action: Option<Action_game>,
    round: usize,
    pub agent_score: i32,
    pub adversary_score: i32,
    pub rewards: Vec<Vec<Vec<Reward>>>,
    pub probabilities: Vec<Vec<Vec<f64>>>,
    pub all_position: Vec<State>,
    pub all_actions: Vec<ActionType>,
    pub terminal_position: Vec<State>,
}

impl RPSGame {
    pub fn new() -> Self {
        let rewards = Self::generate_rewards();
        let probabilities = Self::generate_probabilities();
        let all_position = (0..=2).collect();
        let all_actions = vec![0, 1, 2];
        let terminal_position = vec![2];

        RPSGame {
            agent_action: None,
            adversary_action: None,
            first_agent_action: None,
            round: 0,
            agent_score: 0,
            adversary_score: 0,
            rewards,
            probabilities,
            all_position,
            all_actions,
            terminal_position,
        }
    }

    fn generate_rewards() -> Vec<Vec<Vec<Reward>>> {
        let mut rewards = vec![vec![vec![0.0; 3]; 3]; 3];
        for state in 0..3 {
            for action in 0..3 {
                let agent_action = match action {
                    0 => Action_game::Rock,
                    1 => Action_game::Paper,
                    2 => Action_game::Scissors,
                    _ => panic!("Invalid action"),
                };
                for next_state in 0..3 {
                    let adversary_action = match next_state {
                        0 => Action_game::Rock,
                        1 => Action_game::Paper,
                        2 => Action_game::Scissors,
                        _ => panic!("Invalid state"),
                    };
                    rewards[state][action][next_state] = agent_action.beats(adversary_action) as Reward;
                }
            }
        }
        rewards
    }

    fn generate_probabilities() -> Vec<Vec<Vec<f64>>> {
        let mut probabilities = vec![vec![vec![0.0; 3]; 3]; 3];
        for state in 0..3 {
            for action in 0..3 {
                for next_state in 0..3 {
                    probabilities[state][action][next_state] = 1.0 / 3.0;
                }
            }
        }
        probabilities
    }

    pub fn choose_adversary_action(&self) -> Action_game {
        if self.round == 0 {
            let actions = [Action_game::Rock, Action_game::Paper, Action_game::Scissors];
            let random_index = rand::thread_rng().gen_range(0..3);
            actions[random_index]
        } else {
            self.first_agent_action.unwrap()
        }
    }
}

impl Environment for RPSGame {
    fn reset(&mut self) -> State {
        self.agent_action = None;
        self.adversary_action = None;
        self.first_agent_action = None;
        self.round = 0;
        self.agent_score = 0;
        self.adversary_score = 0;
        0 // Return initial state ID
    }

    fn step(&mut self, action: ActionType) -> (State, Reward, bool) {
        let agent_action = match action {
            0 => Action_game::Rock,
            1 => Action_game::Paper,
            2 => Action_game::Scissors,
            _ => panic!("Invalid action"),
        };

        self.agent_action = Some(agent_action);

        if self.round == 0 {
            self.first_agent_action = Some(agent_action);
        }

        self.adversary_action = Some(self.choose_adversary_action());

        let result = self.agent_action.unwrap().beats(self.adversary_action.unwrap());
        self.agent_score += result;
        self.round += 1;

        let done = self.round >= 2;
        (self.round as State, result as Reward, done)
    }

    fn available_actions(&self) -> Vec<ActionType> {
        vec![0, 1, 2] // Rock, Paper, Scissors
    }

    fn all_states(&self) -> Vec<State> {
        self.all_position.clone()
    }

    fn terminal_states(&self) -> Vec<State> {
        self.terminal_position.clone()
    }

    fn set_state(&mut self, state: State) {
        self.round = state as usize;
    }

    fn display(&self) {
        println!("Round: {}", self.round);
        println!("Agent chose: {:?}", self.agent_action.unwrap());
        println!("Adversary chose: {:?}", self.adversary_action.unwrap());
        println!("Agent score: {}", self.agent_score);
        println!("Adversary score: {}", self.adversary_score);
    }

    fn state_id(&self) -> State {
        self.round as State
    }

    fn score(&self) -> Reward {
        self.agent_score as Reward
    }

    fn is_game_over(&self) -> bool {
        self.round >= 2
    }
}