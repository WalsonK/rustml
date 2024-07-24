use rand::Rng;
use std::fmt;

use crate::environment::environment::{State, Action, Reward, Environment};


pub struct MontyHall {
    pub winning_door: usize,
    pub chosen_door: Option<usize>,
    pub opened_door: Option<usize>,
    pub nb_portes: usize,
    pub rewards: Vec<f32>,
    pub probabilities: Vec<f32>,
    pub all_actions : Vec<Action>,
}

impl MontyHall {
    pub fn new(nb_portes: usize) -> Box<MontyHall> {
        let mut rng = rand::thread_rng();
        let winning_door = rng.gen_range(0..nb_portes);
        println!("La porte gagnante est la porte {}", winning_door);
        let all_actions : Vec<Action> = vec![0; nb_portes];

        let mut monty_hall = MontyHall {
            winning_door,
            chosen_door: None,
            opened_door: None,
            nb_portes,
            rewards: Vec::new(),
            probabilities: Vec::new(),
            all_actions,
        };

        monty_hall.init_rewards();
        monty_hall.init_probabilities();

        Box::new(monty_hall)
    }

    pub fn init_rewards(&mut self) {
        // Initialiser le tableau des récompenses
        self.rewards = vec![0.0; self.nb_portes];
        self.rewards[self.winning_door] = 1.0;
    }

    pub fn init_probabilities(&mut self) {
        // Initialiser les probabilités pour chaque porte
        self.probabilities = vec![1.0 / self.nb_portes as f32; self.nb_portes];
    }
    pub fn valid_action(&self, action: usize) -> bool {
        match self.chosen_door {
            None => action < self.nb_portes,
            Some(_) => action < self.nb_portes,
        }
    }

    fn next_state(&mut self, action: usize) -> (bool, bool) {
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
                    if self.opened_door.unwrap() == action {
                        // Player cannot choose the door that is already opened
                        return (false, false);
                    }
                    self.chosen_door = Some(action);
                }
                (true, true)
            }
        }
    }


    fn reward(&self) -> f32 {
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
    fn done(&self) -> bool {
        self.chosen_door.is_some() && self.opened_door.is_some()
    }

}

impl Environment for MontyHall {
         fn reset(&mut self) -> State{
            let mut rng = rand::thread_rng();
            self.winning_door = rng.gen_range(0..self.nb_portes);
            self.chosen_door = None;
            self.opened_door = None;
            self.init_rewards();
            self.init_probabilities();
             self.state_id()
        }




         fn step(&mut self, action: usize) -> (State, Reward, bool) {
            assert!(!self.is_game_over());
            assert!(self.available_actions().contains(&action));

            if let Some(chosen_door) = self.chosen_door {
                if self.opened_door.is_some() && chosen_door == action {
                    // Player decides not to switch doors
                    return (self.state_id(), 0.0, false);
                }
            }

            let (success, _) = self.next_state(action);
            if !success {
                return (self.state_id(), 0.0, false); // Return 0 reward and false if the action is invalid
            }

            if let Some(chosen_door) = self.chosen_door {
                if chosen_door != action {
                    // Player decides to switch doors
                    self.chosen_door = Some(action);
                }
            }

             (self.state_id(), 0.0, self.is_game_over())
        }


         fn available_actions(&self) -> Vec<usize> {
            match self.opened_door {
                None => (0..self.nb_portes).collect(),
                Some(opened) => (0..self.nb_portes).filter(|&x| x != opened).collect(),
            }
        }

         fn is_game_over(&self) -> bool {
            self.done()
        }

         fn score(&self) -> f32 {
            if let (Some(chosen), Some(_)) = (self.chosen_door, self.opened_door) {
                if chosen == self.winning_door {
                    return 1.0;
                } else {
                    return 0.0;
                }
            }
            0.0
        }

    fn all_states(&self) -> Vec<State> {
        (0..self.nb_portes as State).collect()
    }


    fn terminal_states(&self) -> Vec<State> {
        todo!()
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


    fn all_action(&self) -> Vec<Action> {
        self.all_actions.iter().map(|&action| action as Action).collect()
    }

    fn is_forbidden(&self, state_or_action: usize) -> bool{
        false
    }

    fn transition_probability(&self, state: usize, action: usize, next_state: usize, reward: usize) -> f32 {
        todo!()
    }

    fn random_state(&mut self) {
        todo!()
    }
}





impl fmt::Display for MontyHall {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_monty_hall() {
        let monty_hall = MontyHall::new(3);
        assert_eq!(monty_hall.nb_portes, 3);
        assert!(monty_hall.winning_door < 3);
    }

    #[test]
    fn test_reset() {
        let mut monty_hall = MontyHall::new(3);
        monty_hall.reset();
        assert!(monty_hall.chosen_door.is_none());
        assert!(monty_hall.opened_door.is_none());
    }

    #[test]
    fn test_valid_action() {
        let mut monty_hall = MontyHall::new(3);
        monty_hall.chosen_door = None;
        assert!(monty_hall.valid_action(0));
        assert!(monty_hall.valid_action(1));
        assert!(monty_hall.valid_action(2));

        monty_hall.chosen_door = Some(1);
        assert!(monty_hall.valid_action(0));
        assert!(monty_hall.valid_action(2));
        assert!(monty_hall.valid_action(1));
        assert!(!monty_hall.valid_action(3));
    }

    #[test]
    fn test_next_state() {
        let mut monty_hall = MontyHall::new(3);
        let (success, updated) = monty_hall.next_state(1);
        assert!(success);
        assert!(!updated);

        let (success, updated) = monty_hall.next_state(0);
        assert!(success);
        assert!(updated);
    }

    #[test]
    fn test_reward() {
        let mut monty_hall = MontyHall::new(3);
        monty_hall.winning_door = 0;
        monty_hall.chosen_door = Some(1);
        monty_hall.opened_door = Some(2);
        assert_eq!(monty_hall.reward(), 0.0);

        monty_hall.chosen_door = Some(0);
        assert_eq!(monty_hall.reward(), 1.0);
    }

    #[test]
    /*fn test_step() {
        let mut monty_hall = MontyHall::new(3);
        let winning_door = monty_hall.winning_door;
        monty_hall.chosen_door = None;
        let (reward, done) = monty_hall.step(1);
        assert_eq!(reward, 0.0);
        assert!(!done);

        monty_hall.chosen_door = Option::from(monty_hall.winning_door);
        let (reward, done) = monty_hall.step(winning_door);
        assert_eq!(reward, 1.0);
        assert!(done);
    }*/

    #[test]
    fn test_available_actions() {
        let mut monty_hall = MontyHall::new(3);
        assert_eq!(monty_hall.available_actions(), vec![0, 1, 2]);

        monty_hall.opened_door = Some(1);
        assert_eq!(monty_hall.available_actions(), vec![0, 2]);
    }

    #[test]
    fn test_is_game_over() {
        let mut monty_hall = MontyHall::new(3);
        assert!(!monty_hall.is_game_over());

        monty_hall.chosen_door = Some(1);
        monty_hall.opened_door = Some(2);
        assert!(monty_hall.is_game_over());
    }

    #[test]
    fn test_score() {
        let mut monty_hall = MontyHall::new(3);
        monty_hall.winning_door = 0;
        monty_hall.chosen_door = Some(1);
        monty_hall.opened_door = Some(2);
        assert_eq!(monty_hall.score(), 0.0);

        monty_hall.chosen_door = Option::from(monty_hall.winning_door);
        assert_eq!(monty_hall.score(), 1.0);
    }

    #[test]
    fn test_init_rewards() {
        let nb_portes = 3;
        let mut monty_hall = MontyHall::new(nb_portes);

        monty_hall.init_rewards();

        assert_eq!(monty_hall.rewards.len(), nb_portes);
        for i in 0..nb_portes {
            if i == monty_hall.winning_door {
                assert_eq!(monty_hall.rewards[i], 1.0);
            } else {
                assert_eq!(monty_hall.rewards[i], 0.0);
            }
        }
    }

    #[test]
    fn test_init_probabilities() {
        let nb_portes = 3;
        let mut monty_hall = MontyHall::new(nb_portes);

        monty_hall.init_probabilities();

        assert_eq!(monty_hall.probabilities.len(), nb_portes);
        for i in 0..nb_portes {
            assert_eq!(monty_hall.probabilities[i], 1.0 / nb_portes as f32);
        }
    }

    #[test]
    fn test_new_initializes_correctly() {
        let nb_portes = 3;
        let monty_hall = MontyHall::new(nb_portes);

        assert_eq!(monty_hall.rewards.len(), nb_portes);
        for i in 0..nb_portes {
            if i == monty_hall.winning_door {
                assert_eq!(monty_hall.rewards[i], 1.0);
            } else {
                assert_eq!(monty_hall.rewards[i], 0.0);
            }
        }

        assert_eq!(monty_hall.probabilities.len(), nb_portes);
        for i in 0..nb_portes {
            assert_eq!(monty_hall.probabilities[i], 1.0 / nb_portes as f32);
        }
    }

    #[test]
    fn test_reset_initializes_correctly() {
        let nb_portes = 3;
        let mut monty_hall = MontyHall::new(nb_portes);
        monty_hall.reset();

        assert_eq!(monty_hall.rewards.len(), nb_portes);
        for i in 0..nb_portes {
            if i == monty_hall.winning_door {
                assert_eq!(monty_hall.rewards[i], 1.0);
            } else {
                assert_eq!(monty_hall.rewards[i], 0.0);
            }
        }

        assert_eq!(monty_hall.probabilities.len(), nb_portes);
        for i in 0..nb_portes {
            assert_eq!(monty_hall.probabilities[i], 1.0 / nb_portes as f32);
        }
    }
}
