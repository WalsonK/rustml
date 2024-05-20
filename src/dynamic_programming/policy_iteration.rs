
pub struct PolicyEvaluationModel {
    pub num_states: usize,
    pub num_actions: usize,
    pub rewards: usize,
    pub probabilities: Vec<Vec<Vec<f32>>>
}

impl PolicyEvaluationModel {
    fn new(s: usize, a: usize, r: usize, p: Vec<Vec<Vec<f32>>>) -> Box<PolicyEvaluationModel> {
        let mut pe_model = Box::new(PolicyEvaluationModel {
            num_states: s,
            num_actions: a,
            rewards: r,
            probabilities: p, // vec![vec![vec![vec![0.0; 2]; 2]; 2]; 2]
        });

        pe_model
    }

    fn policy_evaluation(&self, theta: f32) {
        let gamma: f32 = 0.999;
        let mut policy = vec![0, self.num_states];
        let mut policy_stable = false;

        while !policy_stable {
            let mut v = vec![0.0, self.num_states as f32];
            let mut delta;

            loop {
                delta = 0.0;
                for state_index in 0..self.num_states {
                    let old_value = v[state_index];
                    let mut new_val = 0.0;
                    for action_index in 0..self.num_actions {
                        let mut action_val = 0.0;
                        for next_state_index in 0..self.num_states {
                            action_val += self.probabilities[state_index][action_index][next_state_index]
                                * (self.rewards[state_index][action_index][next_state_index]
                                + gamma * v[next_state_index]);
                        }
                        new_val = new_val.max(action_val);
                    }
                    v[state_index] = new_val;
                    delta = delta.max((old_value - v[state_index]).abs());
                }
                if delta < theta {
                    break;
                }
            }
        }
    }
}