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

pub struct MonteCarloControl {
    pub epsilon: f64,
    pub gamma: f64,
    pub policy: HashMap<State, HashMap<Action, f64>>,
    pub q_values: HashMap<(State, Action), Reward>,
    pub returns: HashMap<(State, Action), Vec<Reward>>,
}

impl MonteCarloControl {
    pub fn new(epsilon: f64, gamma: f64) -> Box<MonteCarloControl> {
        Box::new(MonteCarloControl {
            epsilon,
            gamma,
            policy: HashMap::new(),
            q_values: HashMap::new(),
            returns: HashMap::new(),
        })
    }

    pub fn initialize_policy<E: Environment>(&mut self, env: &E) {
        for &state in &env.all_states() {
            let mut actions = HashMap::new();
            let available_actions = env.available_actions();
            for &action in &available_actions {
                actions.insert(action, 1.0 / available_actions.len() as f64);
                self.q_values.insert((state, action), 0.0);
                self.returns.insert((state, action), vec![]);
            }
            self.policy.insert(state, actions);
        }
    }

    pub fn choose_action<E: Environment>(&mut self, state: State, env: &E, rng: &mut rand::rngs::ThreadRng) -> Action {
        // If the state is not found in the policy, initialize it
        if !self.policy.contains_key(&state) {
            println!("Initializing policy for new state: {}", state);
            let mut actions = HashMap::new();
            let available_actions = env.available_actions();
            for &action in &available_actions {
                actions.insert(action, 1.0 / available_actions.len() as f64);
                self.q_values.insert((state, action), 0.0);
                self.returns.insert((state, action), vec![]);
            }
            self.policy.insert(state, actions);
        }

        if let Some(action_probs) = self.policy.get(&state) {
            let actions: Vec<&Action> = action_probs.keys().collect();
            let probs: Vec<f64> = action_probs.values().copied().collect();
            **actions.choose_weighted(rng, |&action| probs[actions.iter().position(|&&a| a == *action).unwrap()]).unwrap()
        } else {
            panic!("No entry found for state: {}", state);
        }
    }

    pub fn on_policy_mc_control<E: Environment>(&mut self, env: &mut E, num_episodes: usize, max_steps: usize) {
        let mut rng = thread_rng();
        self.initialize_policy(env);

        for _ in 0..num_episodes {
            let mut episode: Vec<EpisodeStep> = vec![];
            let mut state = env.reset();
            let mut done = false;
            let mut steps = 0;

            while steps < max_steps {
                let action = self.choose_action(state, env, &mut rng);
                let (next_state, reward, is_done) = env.step(action);
                episode.push(EpisodeStep { state, action, reward });
                state = next_state;
                done = is_done;
                steps += 1;
            }

            self.process_episode(episode);
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
        let mut best_value = f64::NEG_INFINITY;

        for &action in self.policy[&state].keys() {
            let value = *self.q_values.get(&(state, action)).unwrap_or(&f64::NEG_INFINITY);
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
}
