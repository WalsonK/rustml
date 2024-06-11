pub struct ValueIterationModel {
    pub num_states: usize,
    pub num_actions: usize,
    pub rewards: usize,
    pub probabilities: Vec<Vec<Vec<f32>>>,
    pub gamma: f32,
    pub policy: Vec<f32>,
    pub value_function: Vec<f32>
}

impl ValueIterationModel {
    fn new(s: usize, a: usize, r: usize, p: Vec<Vec<Vec<f32>>>, g: f32) -> Box<ValueIterationModel>{
        let mut vi_model = Box::new(ValueIterationModel {
            num_states: s,
            num_actions: a,
            rewards: r,
            probabilities: p,
            gamma: g,
            policy: vec![0.0; s],
            value_function: vec![0.0; s]
        });

        vi_model
    }

    fn iteration(&mut self, theta: f32) {
        let mut delta: f32 = 0.0;

        loop {
            for state_index in 0..self.num_states {
                let old_value = self.value_function[state_index];
                let mut max_value: f32 = -99999.9;
                let mut best_action: usize = 0;

                for action_index in 0..self.num_actions {
                    let mut total: f32 = 0.0;
                    for next_state in 0..self.num_states {
                        total += self.probabilities[state_index][action_index][next_state]
                            * (self.rewards[state_index][action_index][next_state]
                            + self.gamma * self.policy[next_state])
                    }
                    if total > max_value {
                        max_value = total;
                        best_action = action_index;
                    }
                }
                self.policy[state_index] = best_action as f32;
                self.value_function[state_index] = max_value;
                delta = delta.max((old_value - self.value_function[state_index]).abs())
            }

            if delta < theta { break; }
        }

    }
}