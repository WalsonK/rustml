extern crate rand;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use std::collections::HashMap;
use rand::prelude::IteratorRandom;
use crate::environment::environment::{State, Action, Reward, Environment};

#[derive(Clone, Debug)]
pub struct EpisodeStep {
    pub state: State,
    pub action: Action,
    pub reward: Reward,
}

pub struct DynaQModel {
    pub iterations: usize,
    pub gamma: f64,
    pub alpha: f64,
    pub epsilon: f64,
    pub planning_steps: usize, // Number of planning steps
    pub q_values: HashMap<(State, Action), Reward>,
    pub model: HashMap<(State, Action), (Reward, State)>,
}

impl DynaQModel {
    pub fn new(iterations: usize, gamma: f64, alpha: f64, epsilon: f64, planning_steps: usize) -> Box<DynaQModel> {
        Box::new(DynaQModel {
            iterations,
            gamma,
            alpha,
            epsilon,
            planning_steps,
            q_values: HashMap::new(),
            model: HashMap::new(),
        })
    }

    pub fn dyna_q<E: Environment>(&mut self, env: &mut E) {
        let mut rng = thread_rng();

        for _ in 0..self.iterations {
            // Get current nonterminal state S
            let mut state = env.reset();

            while true {
                // Choose action A using epsilon-greedy policy
                let available_actions = env.available_actions();
                let action = self.epsilon_greedy(state, &available_actions, &mut rng);

                // Take action A, observe reward R and next state S'
                let (next_state, reward, done) = env.step(action);

                // Update Q-value
                let max_q_next = self.max_q_value(next_state, &available_actions);
                let q = self.q_values.entry((state, action)).or_insert(0.0);
                *q += self.alpha * (reward + self.gamma * max_q_next - *q);

                // Update model
                self.model.insert((state, action), (reward, next_state));

                // Planning phase
                for _ in 0..self.planning_steps {
                    if let Some((&(s, a), &(r, s_prime))) = self.model.iter().choose(&mut rng) {
                        let max_q_s_prime = self.max_q_value(s_prime, &available_actions);
                        let q_sa = self.q_values.entry((s, a)).or_insert(0.0);
                        *q_sa += self.alpha * (r + self.gamma * max_q_s_prime - *q_sa);
                    }
                }

                // Move to the next state
                state = next_state;

                if done {
                    break;
                }
            }
        }
    }

    fn max_q_value(&self, state: State, actions: &[Action]) -> f64 {
        actions
            .iter()
            .map(|&action| *self.q_values.get(&(state, action)).unwrap_or(&0.0))
            .fold(std::f64::MIN, |a, b| a.max(b))
    }

    fn epsilon_greedy(&self, state: State, actions: &[Action], rng: &mut rand::prelude::ThreadRng) -> Action {
        if rng.gen::<f64>() < self.epsilon {
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
                if q_value > *self.q_values.get(&(state, best_action)).unwrap_or(&f64::NEG_INFINITY) {
                    policy.insert(state, action);
                }
            } else {
                policy.insert(state, action);
            }
        }

        policy
    }

    pub fn print_policy(&self, policy: &HashMap<State, Action>) {
        let mut policy_dict = HashMap::new();

        for (state, action) in policy {
            policy_dict.insert(state, action);
        }

        println!("Policy: {:?}", policy_dict);
    }

}