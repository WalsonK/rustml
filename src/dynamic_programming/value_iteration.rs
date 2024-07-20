use std::collections::HashMap;
use std::fs::File;
use std::{io};
use rand::Rng;
use serde_json;
use crate::environment::environment::{Action, Reward, State};

pub struct ValueIterationModel {
    pub states: Vec<usize>, // Changed to usize for consistency
    pub actions: Vec<usize>, // Changed to usize for consistency
    pub rewards: Vec<Vec<Vec<Reward>>>,
    pub probabilities: Vec<Vec<Vec<f32>>>,
    pub gamma: f32,
    pub policy: Vec<usize>, // Changed to usize for consistency
    pub value_function: Vec<f32>,
}

impl ValueIterationModel {
    pub fn new(s: Vec<usize>, a: Vec<usize>, r: Vec<Vec<Vec<Reward>>>, p: Vec<Vec<Vec<f32>>>, g: f32, terminal_state: Vec<usize>) -> Box<ValueIterationModel> {
        let mut rng = rand::thread_rng();
        let mut vi_model = Box::new(ValueIterationModel {
            states: s.clone(),
            actions: a,
            rewards: r,
            probabilities: p,
            gamma: g,
            policy: vec![0; s.len()],
            value_function: (0..s.len()).map(|_| rng.gen::<f32>()).collect(),
        });
        for &s in terminal_state.iter() {
            vi_model.value_function[s] = 0.0;
        }
        vi_model
    }

    pub fn iteration(&mut self, theta: f32) {
        let mut delta: f32;

        loop {
            delta = 0.0;

            for state_index in 0..self.states.len() {
                let old_value = self.value_function[state_index];
                let mut max_value = f32::NEG_INFINITY;
                let mut best_action = 0;

                for action_index in 0..self.actions.len() {
                    let mut total = 0.0;
                    for next_state in 0..self.states.len() {
                        total += self.probabilities[state_index][action_index][next_state]
                            * (self.rewards[state_index][action_index][next_state]
                            + self.gamma * self.value_function[next_state]);
                    }
                    if total > max_value {
                        max_value = total;
                        best_action = action_index;
                    }
                }
                self.policy[state_index] = best_action;
                self.value_function[state_index] = max_value;
                delta = delta.max((old_value - self.value_function[state_index]).abs());
            }

            if delta < theta { break; }
        }
    }

    pub fn save_policy(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, &self.policy_to_hashmap())?;
        Ok(())
    }

    pub fn policy_to_hashmap(&self) -> HashMap<usize, usize> { // Changed to usize
        let mut policy_map = HashMap::new();
        for (state, &action) in self.policy.iter().enumerate() {
            policy_map.insert(state, action);
        }
        policy_map
    }

    pub fn load_policy(&mut self, filename: &str) -> io::Result<Vec<usize>> { // Changed to usize
        let file = File::open(filename)?;
        let map: HashMap<usize, usize> = serde_json::from_reader(file)?;
        let max_key = match map.keys().max() {
            Some(&key) => key,
            None => 0,
        };
        let mut vec = vec![0; max_key + 1];
        for (key, value) in map {
            vec[key] = value;
        }
        self.policy = vec.clone();
        Ok(vec)
    }

    pub fn print_policy(&self) {
        println!("Policy: {:?}", self.policy);
    }
}
