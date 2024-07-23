extern crate rand;
extern crate serde;
extern crate serde_json;

use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use std::collections::HashMap;
use rand::prelude::IteratorRandom;
use crate::environment::environment::{State, Action, Reward, Environment};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write, Read};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpisodeStep {
    pub state: State,
    pub action: Action,
    pub reward: Reward,
}

#[derive(Serialize, Deserialize)]
pub struct DynaQPlusModel {
    pub iterations: usize,
    pub gamma: f32,
    pub alpha: f32,
    pub epsilon: f32,
    pub planning_steps: usize, // Number of planning steps
    pub kappa: f32, // New parameter for Dyna-Q+
    pub q_values: HashMap<(State, Action), Reward>,
    pub model: HashMap<(State, Action), (Reward, State)>,
    pub time_since_last_action: HashMap<(State, Action), usize>, // Track time since last action
    pub policy: HashMap<State, Action>, // Policy map
}

impl DynaQPlusModel {
    pub fn new(iterations: usize, gamma: f32, alpha: f32, epsilon: f32, planning_steps: usize, kappa: f32) -> Box<DynaQPlusModel> {
        Box::new(DynaQPlusModel {
            iterations,
            gamma,
            alpha,
            epsilon,
            planning_steps,
            kappa,
            q_values: HashMap::new(),
            model: HashMap::new(),
            time_since_last_action: HashMap::new(), // Initialize the tracking map
            policy: HashMap::new(), // Initialize policy map
        })
    }

    pub fn dyna_q_plus(&mut self, env: &mut dyn Environment) {
        let mut rng = thread_rng();

        for _ in 0..self.iterations {
            // Get current nonterminal state S
            let mut state = env.reset();

            loop {
                // Choose action A using epsilon-greedy policy
                let available_actions = env.available_actions();
                let action = self.epsilon_greedy(state, &available_actions, &mut rng);

                // Take action A, observe reward R and next state S'
                let (next_state, reward, done) = env.step(action);

                // Update Q-value
                let max_q_next = self.max_q_value(next_state, &available_actions);
                let q = self.q_values.entry((state, action)).or_insert(0.0);
                *q += self.alpha * (reward + (self.gamma * max_q_next) as f32 - *q) as f32;

                // Update model
                self.model.insert((state, action), (reward, next_state));

                // Update time since last action
                self.update_time_since_last_action(state, &available_actions);

                // Reset the time for the current state-action pair
                self.time_since_last_action.insert((state, action), 0);

                // Planning phase
                for _ in 0..self.planning_steps {
                    if let Some((&(s, a), &(r, s_prime))) = self.model.iter().choose(&mut rng) {
                        let max_q_s_prime = self.max_q_value(s_prime, &available_actions);
                        let q_sa = self.q_values.entry((s, a)).or_insert(0.0);
                        let tau_sa = *self.time_since_last_action.get(&(s, a)).unwrap_or(&0);
                        *q_sa += self.alpha * (r + self.kappa * (tau_sa as f32).sqrt() + self.gamma * max_q_s_prime - *q_sa);
                    }
                }

                // Move to the next state
                state = next_state;

                if done {
                    break;
                }
            }
        }
        self.derive_and_assign_policy(); // Update policy after training
    }

    fn max_q_value(&self, state: State, actions: &[Action]) -> f32 {
        actions
            .iter()
            .map(|&action| *self.q_values.get(&(state, action)).unwrap_or(&0.0))
            .fold(std::f64::MIN, |a, b| a.max(b as f64)) as f32
    }

    fn epsilon_greedy(&self, state: State, actions: &[Action], rng: &mut rand::prelude::ThreadRng) -> Action {
        if rng.gen::<f32>() < self.epsilon {
            *actions.choose(rng).unwrap()
        } else {
            actions
                .iter()
                .max_by(|&&a1, &&a2| {
                    self.q_values
                        .get(&(state, a1))
                        .unwrap_or(&0.0)
                        .partial_cmp(self.q_values.get(&(state, a2)).unwrap_or(&0.0))
                        .unwrap()
                })
                .cloned()
                .unwrap_or(actions[0])
        }
    }

    fn update_time_since_last_action(&mut self, state: State, actions: &[Action]) {
        for &action in actions {
            let key = (state, action);
            if let Some(time) = self.time_since_last_action.get_mut(&key) {
                *time += 1;
            } else {
                self.time_since_last_action.insert(key, 1);
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

        policy
    }

    pub fn print_policy(&self) {
        println!("Policy: {:?}", self.policy);
    }

    pub fn save_policy(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, &self.policy)?;
        Ok(())
    }

    pub fn load_policy(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        self.policy = serde_json::from_reader(file)?;
        Ok(())
    }

    pub fn derive_and_assign_policy(&mut self) {
        self.policy = self.derive_policy();
    }
}