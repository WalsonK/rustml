use rand::Rng;
use crate::environment::environment::{State, Action as ActionType, Reward, Environment};
use std::io;

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
    pub fn new() -> Box<RPSGame> {
        let all_position = (0..=2).collect();
        let all_actions = vec![0, 1, 2];
        let terminal_position = vec![2];
        let rewards = vec![vec![vec![0.0; 3]; 3]; 3];
        let probabilities = vec![vec![vec![0.0; 3]; 3]; 3];

        let mut env = Box::new(RPSGame {
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
        });

        env.generate_rewards();
        env.generate_probabilities();
        env
    }

    fn generate_rewards(&mut self) {
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
                    self.rewards[state][action][next_state] = agent_action.beats(adversary_action) as Reward;
                }
            }
        }
    }

    fn generate_probabilities(&mut self) {
        for state in 0..3 {
            for action in 0..3 {
                for next_state in 0..3 {
                    self.probabilities[state][action][next_state] = 1.0 / 3.0;
                }
            }
        }
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

    fn play_two_rounds(&mut self, action: ActionType) {
        for _ in 0..2 {
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

            self.display();

            if self.round >= 2 {
                break;
            }
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

    // L'agent joue deux fois via la nouvelle méthode play_two_rounds
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

        match self.agent_action {
            Some(action) => println!("Agent chose: {:?}", action),
            None => println!("Agent n'a pas encore choisi d'action"),
        }

        match self.adversary_action {
            Some(action) => println!("Adversary chose: {:?}", action),
            None => println!("Adversaire n'a pas encore choisi d'action"),
        }

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