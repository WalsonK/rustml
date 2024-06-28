use rand::Rng;

pub struct ValueIterationModel {
    pub states: Vec<i64>,
    pub actions: Vec<i64>,
    pub rewards: Vec<Vec<Vec<f64>>>,
    pub probabilities: Vec<Vec<Vec<f64>>>,
    pub gamma: f64,
    pub policy: Vec<usize>,
    pub value_function: Vec<f64>,
}

impl ValueIterationModel {
    pub fn new(s: Vec<i64>, a: Vec<i64>, r: Vec<Vec<Vec<f64>>>, p: Vec<Vec<Vec<f64>>>, g: f64, terminal_state: Vec<i64>) -> Box<ValueIterationModel> {
        let mut rng = rand::thread_rng();
        let mut vi_model = Box::new(ValueIterationModel {
            states: s.clone(),
            actions: a,
            rewards: r,
            probabilities: p,
            gamma: g,
            policy: vec![0; s.len()],
            value_function: (0..s.len()).map(|_| rng.gen::<f64>()).collect(),
        });
        for &s in terminal_state.iter() {
            vi_model.value_function[s as usize] = 0.0;
        }
        vi_model
    }

    pub fn iteration(&mut self, theta: f64) {
        let mut delta: f64;

        loop {
            delta = 0.0;

            for state_index in 0..self.states.len() {
                let old_value = self.value_function[state_index];
                let mut max_value = f64::NEG_INFINITY;
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
}
