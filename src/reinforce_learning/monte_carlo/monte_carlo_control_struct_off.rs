extern crate rand;
extern crate serde;
extern crate serde_json;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self};
use crate::environment::environment::{State, Action, Reward, Environment};
/*
#[derive(Clone, Debug)]
pub struct EpisodeStep {
    pub state: State,
    pub action: Action,
    pub reward: Reward,
}

#[derive(Serialize, Deserialize)]
pub struct MonteCarloControlOff {
    pub epsilon: f32,
    pub gamma: f32,
    pub derived_policy: HashMap<State, HashMap<Action, f32>>,
    pub q_values: HashMap<(State, Action), Reward>,
    pub c_values: HashMap<(State, Action), f32>,
    pub policy: HashMap<State, Action>,
}

impl MonteCarloControlOff {
    pub fn new(epsilon: f32, gamma: f32) -> Box<MonteCarloControlOff> {
        Box::new(MonteCarloControlOff {
            epsilon,
            gamma,
            derived_policy: HashMap::new(),
            q_values: HashMap::new(),
            c_values: HashMap::new(),
            policy: HashMap::new(),
        })
    }

    pub fn off_policy_mc_control(&mut self, env: &mut dyn Environment, num_episodes: usize, max_steps: usize) {
        let mut rng = thread_rng();

        for i in 0..num_episodes {
            println!("{}",i);
            let mut episode: Vec<EpisodeStep> = vec![];
            let mut state = env.reset();
            let mut done = false;
            let mut steps = 0;

            // Generate an episode using a soft policy (behavior policy)
            while !done && steps < max_steps {
                let action = self.choose_action_soft(env, state, &mut rng);
                let (next_state, reward, is_done) = env.step(action);
                episode.push(EpisodeStep { state, action, reward });
                state = next_state;
                done = is_done;
                steps += 1;
            }

            self.process_episode_off_policy(episode);
            self.derive_and_assign_policy();
        }
    }

    pub fn choose_action_soft(&mut self, env: &dyn Environment, state: State, rng: &mut rand::rngs::ThreadRng) -> Action {
        if !self.derived_policy.contains_key(&state) {
            let mut actions = HashMap::new();
            let available_actions = env.available_actions();
            for &action in &available_actions {
                actions.insert(action, 1.0 / available_actions.len() as f32);
                self.q_values.insert((state, action), 0.0);
                self.c_values.insert((state, action), 0.0);
            }
            self.derived_policy.insert(state, actions);
        }
        if let Some(action_probs) = self.derived_policy.get(&state) {
            let actions: Vec<&Action> = action_probs.keys().collect();
            let probs: Vec<f32> = action_probs.values().copied().collect();
            **actions.choose_weighted(rng, |&action| probs[actions.iter().position(|&&a| a == *action).unwrap()]).unwrap()
        } else {
            panic!("No entry found for state: {:?}", state);
        }
    }

    fn process_episode_off_policy(&mut self, episode: Vec<EpisodeStep>) {
        let mut g: f32 = 0.0;
        let mut w: f32 = 1.0;

        for step in episode.iter().rev() {
            g = self.gamma * g + step.reward;
            let state_action_pair = (step.state, step.action);

            if let Some(c) = self.c_values.get_mut(&state_action_pair) {
                *c += w;
            }

            if let Some(q) = self.q_values.get_mut(&state_action_pair) {
                if let Some(&c) = self.c_values.get(&state_action_pair) {
                    *q += (w / c) * (g - *q);
                }
            }

            let best_action = self.find_best_action(step.state);
            self.update_policy(step.state, best_action);

            if step.action != best_action {
                break;
            }

            if let Some(action_prob) = self.derived_policy.get(&step.state).and_then(|probs| probs.get(&step.action)) {
                w /= *action_prob;
            }
        }
    }

    fn find_best_action(&self, state: State) -> Action {
        let mut best_action = *self.derived_policy[&state].keys().next().unwrap();
        let mut best_value = f32::NEG_INFINITY;

        for &action in self.derived_policy[&state].keys() {
            let value = *self.q_values.get(&(state, action)).unwrap_or(&f32::NEG_INFINITY);
            if value > best_value {
                best_value = value;
                best_action = action;
            }
        }

        best_action
    }

    fn update_policy(&mut self, state: State, best_action: Action) {
        let actions = self.derived_policy.get_mut(&state).unwrap();
        let num_actions = actions.len() as f32;
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

        for (&state, action_probs) in &self.derived_policy {
            let best_action = action_probs.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).map(|(action, _)| *action).unwrap();
            derived_policy.insert(state, best_action);
        }

        self.policy = derived_policy;
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

    /*pub fn save_q_values(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(filename)?;
        bincode::serialize_into(file, &self.q_values)?;

        Ok(())
    }

    pub fn load_q_values(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(filename)?;
        self.q_values = bincode::deserialize_from(file)?;
        self.derive_and_assign_policy();
        Ok(())
    }*/

    pub fn print_policy(&self) {
        println!("Derived Policy: {:?}", self.policy);
    }
}
*/