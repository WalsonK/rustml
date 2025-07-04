use std::collections::HashMap;
use std::fs::File;
use std::{f32, io};
use rand::Rng;
use crate::environment::environment::{Action, Reward, State};

pub struct PolicyIterationModel {
    pub states: Vec<State>,
    pub actions: Vec<Action>,
    pub rewards: Vec<Vec<Vec<Reward>>>,
    pub probabilities: Vec<Vec<Vec<f32>>>,
    pub gamma: f32,
    pub is_policy_stable: bool,
    pub policy: Vec<Action>,
    pub value_function: Vec<f32>
}

impl PolicyIterationModel {
    pub fn new(states: Vec<State>, actions: Vec<Action>, rewards: Vec<Vec<Vec<Reward>>>,
               probabilities: Vec<Vec<Vec<f32>>>, gamma: f32, terminal_state: Vec<State>) -> Box<PolicyIterationModel> {
        let mut rng = rand::thread_rng();
        let mut pi_model = Box::new(PolicyIterationModel {
            states: states.clone(),
            actions,
            rewards,
            probabilities,
            is_policy_stable: false,
            gamma,
            policy: vec![0; states.len()],
            value_function: (0..states.len()).map(|_| rng.gen::<f32>()).collect()
        });
        for &s in terminal_state.iter() {
            pi_model.value_function[s as usize] = 0.0;
        }
        pi_model
    }

    pub fn policy_evaluation(&mut self, theta: f32){
        loop {
            let mut delta: f32 = 0.0;
            for state in 0..self.states.len() -1 {
                let old_value = self.value_function[state];
                let mut value = 0.0;
                for action in 0..self.actions.len() {
                    for next_state in 0..self.states.len()-1 {
                        value += self.probabilities[state][action][next_state]
                            * (self.rewards[state][action][next_state]
                            + self.gamma * self.policy[next_state] as f32);
                    }
                }
                self.value_function[state] = value;
                delta = delta.max((old_value - self.value_function[state]).abs());
            }
            if delta < theta { break; }
        }
    }

    pub fn policy_improvement(&mut self) -> bool{
        self.is_policy_stable = true;
        for state_index in 0..self.states.len() {
            let old_action = self.policy[state_index];
            let mut best_action: usize = old_action as usize;
            let mut best_action_score = f32::NEG_INFINITY;


            for action_index in 0..self.actions.len() {
                let mut total = 0.0;
                for next_state_index in 0..self.states.len() {
                    total += self.probabilities[state_index][action_index][next_state_index] *
                        (self.rewards[state_index][action_index][next_state_index]
                            + self.gamma * self.policy[next_state_index] as f32)
                }
                if best_action == 0 || total >= best_action_score {
                    best_action = action_index;
                    best_action_score = total;
                }
            }
            self.policy[state_index] = best_action ;
            if self.policy[state_index] != old_action {
                self.is_policy_stable = false;
            }
        }
        return self.is_policy_stable
    }

    pub fn policy_iteration(&mut self) -> &Vec<Action>{
        loop {
            self.policy_evaluation(0.001);
            if self.policy_improvement() { break; }
        }
        return &self.policy
    }

    pub fn policy_to_hashmap(&self) -> HashMap<State, Action> {
        let mut policy_map = HashMap::new();
        for (state, &action) in self.policy.iter().enumerate() {
            policy_map.insert(state as State, action as Action);
        }
        policy_map
    }

    pub fn save_policy(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, &self.policy_to_hashmap())?;
        Ok(())
    }

    pub fn load_policy(&mut self, filename: &str) -> io::Result<Vec<usize>> {
        let file = File::open(filename)?;
        let map: HashMap<i64, i64> = serde_json::from_reader(file)?;
        let max_key = match map.keys().max() {
            Some(&key) => key,
            None => 0,
        };
        let mut vec: Vec<usize> = vec![0; (max_key + 1) as usize];
        for (key, value) in map {
            vec[key as usize] = value as usize;
        }
        self.policy = vec.clone();
        Ok(vec)
    }

    pub fn print_policy(&self) {
        println!("Policy: {:?}", self.policy);
    }

}
