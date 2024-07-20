extern crate rand;
extern crate serde;
extern crate serde_json;

use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use std::collections::HashMap;
use crate::environment::environment::{State, Action, Reward, Environment};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write, Read};

#[derive(Serialize, Deserialize)]
pub struct QLearning {
    pub iterations: usize,
    pub gamma: f32,
    pub alpha: f32,
    pub epsilon: f32,
    pub q_values: HashMap<(State, Action), Reward>,
    pub policy: HashMap<State, Action>, // Policy map
}

impl QLearning {
    pub fn new(iterations: usize, gamma: f32, alpha: f32, epsilon: f32) -> Box<QLearning> {
        Box::new(QLearning {
            iterations,
            gamma,
            alpha,
            epsilon,
            q_values: HashMap::new(),
            policy: HashMap::new(), // Initialize policy map
        })
    }

    pub fn q_learning<E: Environment>(&mut self, env: &mut E) {
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
                *q += self.alpha * (reward + self.gamma * max_q_next - *q);

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
            .fold(std::f32::MIN, |a, b| a.max(b))
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
