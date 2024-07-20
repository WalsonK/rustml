extern crate rand;
extern crate serde;
extern crate serde_json;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use crate::environment::environment::{State, Action, Reward, Environment};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write, Read};

#[derive(Clone, Debug)]
pub struct EpisodeStep {
    pub state: State,
    pub action: Action,
    pub reward: Reward,
}

#[derive(Serialize, Deserialize)]
pub struct MonteCarloControl {
    pub epsilon: f64,
    pub gamma: f32,
    pub policy: HashMap<State, HashMap<Action, f64>>,
    pub q_values: HashMap<(State, Action), Reward>,
    pub returns: HashMap<(State, Action), Vec<Reward>>,
    pub derived_policy: HashMap<State, Action>,
}

impl MonteCarloControl {
    pub fn new(epsilon: f64, gamma: f32) -> Box<MonteCarloControl> {
        Box::new(MonteCarloControl {
            epsilon,
            gamma,
            policy: HashMap::new(),
            q_values: HashMap::new(),
            returns: HashMap::new(),
            derived_policy: HashMap::new(),
        })
    }

    pub fn ensure_policy_initialized<E: Environment>(&mut self, state: State, env: &mut E) {
        if !self.policy.contains_key(&state) {
            let mut actions = HashMap::new();
            let available_actions = env.all_action();
            for &action in &available_actions {
                if !env.is_forbidden(action) {
                    actions.insert(action, 1.0 / available_actions.len() as f64);
                    self.q_values.insert((state, action), 0.0);
                    self.returns.insert((state, action), vec![]);
                }
            }
            self.policy.insert(state, actions);
        }
    }

    pub fn choose_action_soft<E: Environment>(&mut self, state: State, env: &mut E, rng: &mut rand::rngs::ThreadRng) -> Action {
        self.ensure_policy_initialized(state, env);

        let available_actions = env.available_actions();

        if let Some(action_probs) = self.policy.get(&state) {
            let actions: Vec<&Action> = action_probs.keys().collect();
            let probs: Vec<f64> = action_probs.values().copied().collect();

            let mut valid_actions = vec![];
            let mut valid_probs = vec![];
            for (&action, &prob) in actions.iter().zip(probs.iter()) {
                if available_actions.contains(action) && !env.is_forbidden(*action) {
                    valid_actions.push(action);
                    valid_probs.push(prob);
                }
            }

            if valid_actions.is_empty() {
                panic!("No valid actions available for state: {:?}", state);
            }

            match valid_actions.choose_weighted(rng, |&action| valid_probs[valid_actions.iter().position(|&&a| a == *action).unwrap()]) {
                Ok(action) => **action,
                Err(_) => panic!("Failed to choose a weighted action for state: {:?}", state),
            }
        } else {
            panic!("No entry found for state: {:?}", state);
        }
    }

    pub fn on_policy_mc_control<E: Environment>(&mut self, env: &mut E, num_episodes: usize, max_steps: usize) {
        let mut rng = thread_rng();
        let mut i = 0;
        for _ in 0..num_episodes {
            println!("{:?}", i);
            i += 1;
            let mut episode: Vec<EpisodeStep> = vec![];
            let mut state = env.reset();
            let mut done = false;
            let mut steps = 0;
            while !done && steps < max_steps {
                let action = self.choose_action_soft(state, env, &mut rng);
                let (next_state, reward, is_done) = env.step(action);
                episode.push(EpisodeStep { state, action, reward });
                state = next_state;
                done = is_done;
                steps += 1;
            }
            self.process_episode(episode);
            self.derive_and_assign_policy();
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
                self.returns.entry(state_action_pair).or_insert_with(Vec::new).push(g);
                let return_list = &self.returns[&state_action_pair];
                let mean_return = return_list.iter().copied().sum::<Reward>() / return_list.len() as Reward;
                self.q_values.insert(state_action_pair, mean_return);

                let best_action = self.find_best_action(step.state);
                self.update_policy(step.state, best_action);
            }
        }
    }

    fn find_best_action(&self, state: State) -> Action {
        let mut best_action = *self.policy[&state].keys().next().unwrap();
        let mut best_value = f32::NEG_INFINITY;

        for &action in self.policy[&state].keys() {
            let value = *self.q_values.get(&(state, action)).unwrap_or(&f32::NEG_INFINITY);
            if value > best_value {
                best_value = value;
                best_action = action;
            }
        }

        best_action
    }

    fn update_policy(&mut self, state: State, best_action: Action) {
        let actions = self.policy.get_mut(&state).unwrap();
        let num_actions = actions.len() as f64;
        let epsilon = self.epsilon;
        for (&action, prob) in actions.iter_mut() {
            if action == best_action {
                *prob = 1.0 - epsilon + (epsilon / num_actions);
            } else {
                *prob = epsilon / num_actions;
            }
        }
    }

    pub fn derive_and_assign_policy(&mut self) {
        let mut derived_policy = HashMap::new();

        for (&state, action_probs) in &self.policy {
            let best_action = action_probs.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).map(|(action, _)| *action).unwrap();
            derived_policy.insert(state, best_action);
        }

        self.derived_policy = derived_policy;
    }

    pub fn save_policy(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, &self.derived_policy)?;
        Ok(())
    }

    pub fn load_policy(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        self.derived_policy = serde_json::from_reader(file)?;
        Ok(())
    }

    pub fn print_policy(&self) {
        println!("Derived Policy: {:?}", self.derived_policy);
    }
}
