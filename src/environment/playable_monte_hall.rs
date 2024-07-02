extern crate rand;
use rand::Rng;
use crate::environment::environment::{State, Action, Reward, Environment};
use std::fmt;

pub struct playable_MontyHall {
    pub winning_door: usize,
    pub chosen_door: Option<usize>,
    pub opened_door: Option<usize>,
    pub nb_portes: usize,
    pub rewards: Vec<f32>,
    pub probabilities: Vec<f32>,
}

impl playable_MontyHall {
    pub fn new(nb_portes: usize) -> Box<playable_MontyHall> {
        let mut rng = rand::thread_rng();
        let winning_door = rng.gen_range(0..nb_portes);
        println!("La porte gagnante est la porte {}", winning_door);

        let mut monty_hall = playable_MontyHall {
            winning_door,
            chosen_door: None,
            opened_door: None,
            nb_portes,
            rewards: Vec::new(),
            probabilities: Vec::new(),
        };

        monty_hall.init_rewards();
        monty_hall.init_probabilities();

        Box::new(monty_hall)
    }

    pub fn init_rewards(&mut self) {
        self.rewards = vec![0.0; self.nb_portes];
        self.rewards[self.winning_door] = 1.0;
    }

    pub fn init_probabilities(&mut self) {
        self.probabilities = vec![1.0 / self.nb_portes as f32; self.nb_portes];
    }

    pub fn valid_action(&self, action: usize) -> bool {
        match self.chosen_door {
            None => action < self.nb_portes,
            Some(_) => action < self.nb_portes,
        }
    }

    pub fn next_state(&mut self, action: usize) -> (bool, bool) {
        if !self.valid_action(action) {
            return (false, false);
        }
        match self.chosen_door {
            None => {
                self.chosen_door = Some(action);
                (true, false)
            }
            Some(_) => {
                if self.opened_door.is_none() {
                    let mut rng = rand::thread_rng();
                    let unopened_doors: Vec<usize> = (0..self.nb_portes)
                        .filter(|&x| x != self.winning_door && x != self.chosen_door.unwrap())
                        .collect();
                    let opened_door = unopened_doors[rng.gen_range(0..unopened_doors.len())];
                    self.opened_door = Some(opened_door);
                } else {
                    self.chosen_door = Some(action);
                }
                (true, true)
            }
        }
    }

    pub fn reward(&self) -> f64 {
        match (self.chosen_door, self.opened_door) {
            (Some(chosen), Some(_)) => {
                if chosen == self.winning_door {
                    1.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }


}

impl Environment for playable_MontyHall {
    fn reset(&mut self) -> State {
        let mut rng = rand::thread_rng();
        self.winning_door = rng.gen_range(0..self.nb_portes);
        self.chosen_door = None;
        self.opened_door = None;
        self.init_rewards();
        self.init_probabilities();
        self.state_id()
    }

    fn step(&mut self, action: Action) -> (State, Reward, bool) {
        let action = action as usize;
        let (success, updated) = self.next_state(action);
        if !success {
            return (self.state_id(), 0.0, false);
        }
        if updated {
            return (self.state_id(), self.reward() as Reward, self.is_game_over());
        }
        (self.state_id(), 0.0, self.is_game_over())
    }

    fn available_actions(&self) -> Vec<Action> {
        match self.opened_door {
            None => (0..self.nb_portes as Action).collect(),
            Some(opened) => (0..self.nb_portes as Action).filter(|&x| x != opened as Action).collect(),
        }
    }

    fn all_states(&self) -> Vec<State> {
        (0..self.nb_portes as State).collect()
    }

    fn set_state(&mut self, state: State) {
        self.chosen_door = Some(state as usize);
    }

    fn display(&self) {
        println!("{}", self);
    }

    fn state_id(&self) -> State {
        self.chosen_door.unwrap_or(self.nb_portes) as State
    }

    fn score(&self) -> Reward {
        self.reward() as Reward
    }
   fn is_game_over(&self) -> bool {
        self.chosen_door.is_some() && self.opened_door.is_some()
    }
}

impl fmt::Display for playable_MontyHall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chosen_door = self
            .chosen_door
            .map_or("None".to_string(), |x| x.to_string());
        let opened_door = self
            .opened_door
            .map_or("None".to_string(), |x| x.to_string());
        write!(
            f,
            "Winning door: {}, Chosen door: {}, Opened door: {}",
            self.winning_door, chosen_door, opened_door
        )
    }
}
