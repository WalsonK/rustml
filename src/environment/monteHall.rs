use rand::Rng;
use std::fmt;
use crate::environment::environment::Environment;

type State = usize;
type Action = usize;
type Reward = f32;

pub struct MonteHall {
    pub winning_door: usize,
    pub chosen_door: Option<usize>,
    pub opened_door: Option<usize>,
    pub nb_portes: usize,
    pub state: State,
}

impl MonteHall {
    pub fn new(nb_portes: usize) -> Box<MonteHall> {
        let mut rng = rand::thread_rng();
        let winning_door = rng.gen_range(0..nb_portes);
        println!("La porte gagnante est la porte {}", winning_door);

        Box::new(MonteHall {
            winning_door,
            chosen_door: None,
            opened_door: None,
            nb_portes,
            state: 1, // Start state
        })
    }

    fn choose_door(&mut self, door: usize) {
        if door >= 1 && door <= self.nb_portes {
            self.chosen_door = Some(door - 1); // Adjusting to 0-based index
            self.open_door(); // Directly transition to opening a door
        } else {
            panic!("Invalid door choice");
        }
    }

    fn open_door(&mut self) {
        if self.opened_door.is_none() {
            let mut rng = rand::thread_rng();
            let unopened_doors: Vec<usize> = (0..self.nb_portes)
                .filter(|&x| x != self.winning_door && x != self.chosen_door.unwrap())
                .collect();
            let opened_door = unopened_doors[rng.gen_range(0..unopened_doors.len())];
            self.opened_door = Some(opened_door);
            self.state = match (self.chosen_door.unwrap(), opened_door) {
                (0, 1) => 4,
                (0, 2) => 5,
                (1, 0) => 6,
                (1, 2) => 7,
                (2, 0) => 8,
                (2, 1) => 9,
                _ => panic!("Invalid state transition"),
            };
        }
    }

    fn switch_choice(&mut self) {
        if let Some(chosen) = self.chosen_door {
            let remaining_doors: Vec<usize> = (0..self.nb_portes)
                .filter(|&x| x != chosen && x != self.opened_door.unwrap())
                .collect();
            self.chosen_door = Some(remaining_doors[0]);
        }
    }

    fn reward(&self) -> f32 {
        if self.chosen_door == Some(self.winning_door) {
            1.0
        } else {
            0.0
        }
    }

    fn done(&self) -> bool {
        self.state >= 10 // Final states from 10 to 21
    }

    pub fn play_game(&mut self, initial_choice: usize, switch: bool) -> f32 {
        self.choose_door(initial_choice);
        if switch {
            self.switch_choice();
            self.state += 12; // Transition to the final state for switching
        } else {
            self.state += 6; // Transition to the final state for keeping
        }
        self.reward()
    }
}

impl Environment for MonteHall {

    fn reset(&mut self) -> State {
        let mut rng = rand::thread_rng();
        self.winning_door = rng.gen_range(0..self.nb_portes);
        self.chosen_door = None;
        self.opened_door = None;
        self.state = 1; // Start state
        self.state
    }

    fn step(&mut self, action: Action) -> (State, Reward, bool) {
        match self.state {
            1 | 2 | 3 => {
                if action >= 1 && action <= self.nb_portes {
                    self.choose_door(action);
                    (self.state, 0.0, false)
                } else {
                    panic!("Invalid action in state 1-3");
                }
            }
            4 | 5 | 6 | 7 | 8 | 9 => {
                if action == 4 {
                    // Keep the door
                    self.state += 6; // Transition to the final state for keeping
                } else if action == 5 {
                    // Switch the door
                    self.switch_choice();
                    self.state += 12; // Transition to the final state for switching
                } else {
                    panic!("Invalid action in state 4-9");
                }
                let reward = self.reward();
                (self.state, reward, true)
            }
            _ => (self.state, 0.0, true), // Terminal states
        }
    }

    fn available_actions(&self) -> Vec<Action> {
        match self.state {
            1 | 2 | 3 => (1..=self.nb_portes).collect(), // Choosing any door
            4 | 5 | 6 | 7 | 8 | 9 => vec![4, 5], // Deciding to switch or not
            _ => vec![], // No actions available in the terminal state
        }
    }

    fn is_game_over(&self) -> bool {
        self.done()
    }

    fn score(&self) -> f32 {
        self.reward()
    }

    fn all_states(&self) -> Vec<State> {
        (1..=21).collect() // Choosing, intermediate, and terminal states
    }

    fn terminal_states(&self) -> Vec<State> {
        (10..=21).collect() // Terminal states
    }

    fn set_state(&mut self, state: State) {
        self.state = state;
        match state {
            1 | 2 | 3 => {
                // State where the door is chosen but not opened
                self.opened_door = None;
            }
            4 | 5 | 6 | 7 | 8 | 9 => {
                // Ensure door opening state consistency
                self.open_door();
            }
            10..=21 => {
                // Final state, no changes needed
            }
            _ => panic!("Invalid state"),
        }
    }

    fn display(&self) {
        println!("{}", self);
    }

    fn state_id(&self) -> State {
        self.state
    }

    fn all_action(&self) -> Vec<Action> {
        vec![1, 2, 3, 4, 5] // 1, 2, 3 for doors; 4 for keep, 5 for switch
    }

    fn is_forbidden(&self, _state_or_action: usize) -> bool {
        false
    }

    fn transition_probability(&self, _state: usize, _action: usize, _next_state: usize, _reward: usize) -> f32 {
        todo!()
    }

    fn random_state(&mut self) {
        todo!()
    }
}

impl fmt::Display for MonteHall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chosen_door = self.chosen_door.map_or("None".to_string(), |x| (x + 1).to_string());
        let opened_door = self.opened_door.map_or("None".to_string(), |x| (x + 1).to_string());
        write!(
            f,
            "Winning door: {}, Chosen door: {}, Opened door: {}, State: {}",
            self.winning_door + 1, chosen_door, opened_door, self.state
        )
    }
}

