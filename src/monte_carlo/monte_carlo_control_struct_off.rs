extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use crate::environment::environment::{State, Action, Reward, Environment};

#[derive(Clone, Debug)]
pub struct EpisodeStep {
    pub state: State,
    pub action: Action,
    pub reward: Reward,
}

pub struct MonteCarloControlOff {
    pub epsilon: f32,
    pub gamma: f32,
    pub policy: HashMap<State, HashMap<Action, f32>>,
    pub q_values: HashMap<(State, Action), Reward>,
    pub c_values: HashMap<(State, Action), f32>,
}

impl MonteCarloControlOff {
    pub fn new(epsilon: f32, gamma: f32) -> Box<MonteCarloControlOff> {
        Box::new(MonteCarloControlOff {
            epsilon,
            gamma,
            policy: HashMap::new(),
            q_values: HashMap::new(),
            c_values: HashMap::new(),
        })
    }

    pub fn initialize_policy<E: Environment>(&mut self, env: &E) {
        for &state in &env.all_states() {
            let mut actions = HashMap::new();
            let available_actions = env.available_actions();
            for &action in &available_actions {
                actions.insert(action, 1.0 / available_actions.len() as f32);
                self.q_values.insert((state, action), 0.0);
                self.c_values.insert((state, action), 0.0);
            }
            self.policy.insert(state, actions);
        }
    }

    pub fn off_policy_mc_control<E: Environment>(&mut self, env: &mut E, num_episodes: usize, max_steps: usize) {
        let mut rng = thread_rng();
        self.initialize_policy(env);

        for _ in 0..num_episodes {
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
        }
    }

    pub fn choose_action_soft<E: Environment>(&mut self, env: &E, state: State, rng: &mut rand::rngs::ThreadRng) -> Action {
        if !self.policy.contains_key(&state) {
            println!("Initializing policy for new state: {:?}", state);
            let mut actions = HashMap::new();
            let available_actions = env.available_actions();
            for &action in &available_actions {
                actions.insert(action, 1.0 / available_actions.len() as f32);
                self.q_values.insert((state, action), 0.0);
                self.c_values.insert((state, action), 0.0);
            }
            self.policy.insert(state, actions);
        }
        if let Some(action_probs) = self.policy.get(&state) {
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
            g = (self.gamma as f32) * g + (step.reward as f32);
            let state_action_pair = (step.state, step.action);

            if let Some(c) = self.c_values.get_mut(&state_action_pair) {
                *c += w;
            }

            if let Some(q) = self.q_values.get_mut(&state_action_pair) {
                if let Some(c) = self.c_values.get(&state_action_pair).map(|&v| v as f32) {
                    *q += ((w / c) * (g - *q as f32)) as f32 ;
                }
            }

            let best_action = self.find_best_action(step.state);
            self.update_policy(step.state, best_action);

            if step.action != best_action {
                break;
            }

            if let Some(action_prob) = self.policy.get(&step.state).and_then(|probs| probs.get(&step.action)) {
                w /= *action_prob;
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
}
