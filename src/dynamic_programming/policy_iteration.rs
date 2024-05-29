
pub struct PolicyEvaluationModel {
    pub num_states: usize,
    pub num_actions: usize,
    pub rewards: usize,
    pub probabilities: Vec<Vec<Vec<f32>>>,
    pub is_policy_stable: bool
}

impl PolicyEvaluationModel {
    fn new(s: usize, a: usize, r: usize, p: Vec<Vec<Vec<f32>>>) -> Box<PolicyEvaluationModel> {
        let mut pe_model = Box::new(PolicyEvaluationModel {
            num_states: s,
            num_actions: a,
            rewards: r,
            probabilities: p, // vec![vec![vec![vec![0.0; 2]; 2]; 2]; 2]
            is_policy_stable: false
        });

        pe_model
    }

    fn policy_evaluation(&self, theta: f32) -> Vec<f32> {
        let gamma: f32 = 0.999;
        let mut v = vec![0.0; self.num_states];


        loop {
            let mut delta = 0.0;

            for state_index in 0..self.num_states {
                let old_value = v[state_index];
                let mut action_val = 0.0;

                for action_index in 0..self.num_actions {
                    for next_state_index in 0..self.num_states {
                        action_val += self.probabilities[state_index][action_index][next_state_index]
                            * (self.rewards[state_index][action_index][next_state_index]
                            + gamma * v[next_state_index]);
                    }
                }
                v[state_index] = action_val;
                delta = delta.max((old_value - v[state_index]).abs());
            }

            if delta < theta { break; }
        }
        v
    }

    fn policy_improvement(&mut self, policy: Vec<i32>) {
        self.is_policy_stable = true;
        for s in 0..self.num_states {
            let old_action = policy[s];

        }
    }
}