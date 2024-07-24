use rand::Rng;
use crate::environment::environment::{State, Action, Reward, Environment};

pub struct ValueIterationModel {
    pub states: Vec<State>,
    pub actions: Vec<Action>,
    pub gamma: f32,
    pub policy: Vec<usize>,
    pub value_function: Vec<Reward>,
}

impl ValueIterationModel {
    pub fn new(s: Vec<State>, a: Vec<Action>, g: f32, terminal_state: Vec<State>) -> Box<ValueIterationModel> {
        let mut rng = rand::thread_rng();
        let mut vi_model = Box::new(ValueIterationModel {
            states: s.clone(),
            actions: a,
            gamma: g,
            policy: vec![0; s.len()],
            value_function: (0..s.len()).map(|_| rng.gen::<f32>()).collect(),
        });
        for &s in terminal_state.iter() {
            vi_model.value_function[s as usize] = 0.0;
        }
        vi_model
    }

    pub fn iteration<E: Environment>(&mut self, env: &mut E, theta: f32) {
        let mut delta: f32;

        loop {
            delta = 0.0;

            for state_index in 0..self.states.len() {
                let old_value = self.value_function[state_index];
                let mut max_value = f32::NEG_INFINITY;
                let mut best_action = 0;
                println!("{}",state_index);
                for action_index in 0..self.actions.len() {

                    let mut total = 0.0;
                    for next_state in 0..self.states.len() {
                        let prob = env.transition_probability(state_index, action_index, next_state, 0); // Assuming 0 reward for simplicity
                        let reward = env.score();
                        total += prob  * (reward + self.gamma * self.value_function[next_state ]);
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

            println!("Delta : {}",delta);
            if delta < theta { break; }
        }
    }
}