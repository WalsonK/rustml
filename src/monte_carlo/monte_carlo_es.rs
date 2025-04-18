extern crate rand;
extern crate serde;
extern crate serde_json;

use rand::seq::{IteratorRandom, SliceRandom};
use rand::thread_rng;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self};
use crate::environment::environment::{State, Action, Reward, Environment};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpisodeStep {
    pub state: State,
    pub action: Action,
    pub reward: Reward,
}

#[derive(Serialize, Deserialize)]
pub struct MonteCarloESModel {
    pub num_episodes: usize,
    pub gamma: f32,
    pub max_steps: usize,
    pub policy: HashMap<State, Action>,
    pub q_values: HashMap<(State, Action), Reward>,
    pub returns: HashMap<(State, Action), Vec<Reward>>,
}
impl MonteCarloESModel {
    pub fn new(num_episodes: usize, gamma: f32, max_steps: usize) -> Box<MonteCarloESModel> {
        Box::new(MonteCarloESModel {
            num_episodes,
            gamma,
            max_steps,
            policy: HashMap::new(),
            q_values: HashMap::new(),
            returns: HashMap::new(),
        })
    }

    pub fn monte_carlo_es(&mut self, env: &mut dyn Environment) {
        let mut rng = thread_rng();
        let mut i = 0;
        for _ in 0..self.num_episodes {
            env.random_state();
            env.state_id();
            //env.reset();
            println!("{}", i);
            i = i + 1;
            let mut is_first_action = true;
            let mut trajectory: Vec<EpisodeStep> = Vec::new();
            let mut steps_count = 0;

            while steps_count < self.max_steps {
                let state = env.state_id();

                let available_actions = env.available_actions();

                // Assurer que chaque état a une politique initiale
                if !self.policy.contains_key(&state) {
                    let action = available_actions.clone().into_iter().choose(&mut rng).unwrap();
                    self.policy.insert(state, action);
                }

                let action = if is_first_action {
                    is_first_action = false;
                    available_actions.into_iter().choose(&mut rng).unwrap()
                } else {
                    *self.policy.get(&state).unwrap()
                };

                let (new_state, reward, done) = env.step(action);

                trajectory.push(EpisodeStep {
                    state,
                    action,
                    reward,
                });

                steps_count += 1;

                if done {
                    break;
                }
            }

            self.process_episode(trajectory);
        }
    }

    fn process_episode(&mut self, episode: Vec<EpisodeStep>) {
        let mut g: Reward = 0.0;
        let mut visited_state_action_pairs: Vec<(State, Action)> = Vec::new();

        for step in episode.iter().rev() {
            g = self.gamma * g + step.reward;
            let state_action_pair = (step.state, step.action);

            if !visited_state_action_pairs.contains(&state_action_pair) {
                visited_state_action_pairs.push(state_action_pair);
                self.returns.entry(state_action_pair)
                    .or_insert_with(Vec::new)
                    .push(g);

                let return_list = self.returns.get(&state_action_pair).unwrap();
                let mean_return = return_list.iter().copied().sum::<Reward>() / return_list.len() as Reward;
                self.q_values.insert(state_action_pair, mean_return);

                let best_action = self.find_best_action(step.state);
                self.policy.insert(step.state, best_action);
            }
        }
    }

    pub fn derive_policy(&self) -> HashMap<State, Action> {
        let mut policy = HashMap::new();

        for (&(state, action), &q_value) in &self.q_values {
            if let Some(&best_action) = policy.get(&state) {
                if q_value > *self.q_values.get(&(state, best_action)).unwrap_or(&f32::NEG_INFINITY) {
                    policy.insert(state, action);
                }
            } else {
                policy.insert(state, action);
            }
        }

        println!("Derived policy: {:?}", policy);
        policy
    }

    fn find_best_action(&self, state: State) -> Action {
        let mut best_action = 0;
        let mut best_value = f32::NEG_INFINITY;

        for action in self.q_values.keys().filter_map(|&(s, a)| if s == state { Some(a) } else { None }) {
            let value = *self.q_values.get(&(state, action)).unwrap_or(&f32::NEG_INFINITY);
            if value > best_value {
                best_value = value;
                best_action = action;
            }
        }

        best_action
    }

    /* pub fn save_policy(&self, filename: &str) -> io::Result<()> {
         let file = File::create(filename)?;
         serde_json::to_writer(file, &self.policy)?;
         Ok(())
     }

     pub fn load_policy(&mut self, filename: &str) -> io::Result<()> {
         let file = File::open(filename)?;
         self.policy = serde_json::from_reader(file)?;
         Ok(())
     }

     pub fn save_q_values(&self, filename: &str) -> Result<(), Box<dyn Error>> {
         let file = File::create(filename)?;
         bincode::serialize_into(file, &self.q_values)?;
         Ok(())
     }

     pub fn load_q_values(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
         let file = File::open(filename)?;
         self.q_values = bincode::deserialize_from(file)?;
         self.policy = self.derive_policy();
         Ok(())
     }*/
}