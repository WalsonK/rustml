use std::fmt;
use std::ops::RangeInclusive;
use rand::Rng;

pub(crate) const NB_PORTES: usize = 3;

#[derive(Clone, Debug)]
pub struct MontyHall {
    pub winning_door: usize,
    pub chosen_door: Option<usize>,
    pub opened_door: Option<usize>,
}

impl MontyHall {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let winning_door = rng.gen_range(0..NB_PORTES);
        println!("La porte gagnante est la porte {}", winning_door);
        MontyHall {
            winning_door,
            chosen_door: None,
            opened_door: None,
        }
    }

    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        self.winning_door = rng.gen_range(0..NB_PORTES);
        self.chosen_door = None;
        self.opened_door = None;
    }

    fn valid_action(&self, action: usize) -> bool {
        match self.chosen_door {
            None => action < NB_PORTES,
            Some(_) => self.opened_door.is_none(),
        }
    }

    pub(crate) fn next_state(&mut self, action: usize) -> Result<(), ()> {
        if !self.valid_action(action) {
            return Err(());
        }
        match self.chosen_door {
            None => self.chosen_door = Some(action),
            Some(_) => {
                let mut rng = rand::thread_rng();
                let opened_door = (0..NB_PORTES)
                    .filter(|&x| x != self.winning_door && x != self.chosen_door.unwrap())
                    .nth(rng.gen_range(0..NB_PORTES - 1))
                    .unwrap();
                self.opened_door = Some(opened_door);
            }
        }
        Ok(())
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

impl fmt::Display for MontyHall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chosen_door = self.chosen_door.unwrap_or(NB_PORTES);
        let opened_door = self.opened_door.unwrap_or(NB_PORTES);
        write!(
            f,
            "Winning door: {}, Chosen door: {}, Opened door: {}",
            self.winning_door, chosen_door, opened_door
        )
    }
}

pub trait Env {
    type State;
    type Action;
    type Reward;

    fn state(&self) -> &Self::State;
    fn action_range(&self) -> RangeInclusive<Self::Action>;
    fn step(&mut self, action: Self::Action) -> (Self::Reward, bool);
}

impl Env for MontyHall {
    type State = Self;
    type Action = usize;
    type Reward = f32;

    fn state(&self) -> &Self::State {
        self
    }

    fn action_range(&self) -> RangeInclusive<Self::Action> {
        0..=NB_PORTES - 1
    }

    fn step(&mut self, action: Self::Action) -> (Self::Reward, bool) {
        if let Some(chosen_door) = self.chosen_door {
            if chosen_door == action {
                // Le joueur décide de ne pas changer de porte
                return (self.reward(), self.done());
            }
        }
        if let Err(_) = self.next_state(action) {
            return (0.0, false);  // Return 0 reward and false if the action is invalid
        }
        (self.reward(), self.done())
    }
}
