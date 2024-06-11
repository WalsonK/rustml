pub struct PolicyIterationModel {
    pub num_states: usize,
    pub num_actions: usize,
    pub rewards: usize,
    pub probabilities: Vec<Vec<Vec<f32>>>,
    pub gamma: f32,
    pub is_policy_stable: bool,
    pub policy: Vec<f32>
}

impl PolicyIterationModel {
    fn new(s: usize, a: usize, r: usize, p: Vec<Vec<Vec<f32>>>, g: f32) -> Box<PolicyIterationModel> {
        let mut pi_model = Box::new(PolicyIterationModel {
            num_states: s,
            num_actions: a,
            rewards: r,
            probabilities: p, // vec![vec![vec![vec![0.0; 2]; 2]; 2]; 2]
            is_policy_stable: false,
            gamma: g,
            policy: vec![0.0; s]
        });

        pi_model
    }

    fn policy_evaluation(&self, theta: f32) {
        // let gamma: f32 = 0.999;
        loop {
            let mut delta = 0.0;

            for state_index in 0..self.num_states {
                let old_value = self.policy[state_index];
                let mut action_val = 0.0;

                for action_index in 0..self.num_actions {
                    for next_state_index in 0..self.num_states {
                        action_val += self.probabilities[state_index][action_index][next_state_index]
                            * (self.rewards[state_index][action_index][next_state_index]
                            + self.gamma * self.policy[next_state_index]);
                    }
                }
                self.policy[state_index] = action_val;
                delta = delta.max((old_value - self.policy[state_index]).abs());
            }

            if delta < theta { break; }
        }
    }

    fn policy_improvement(&mut self) -> bool{
        self.is_policy_stable = true;
        for state_index in 0..self.num_states {
            let old_action = self.policy[state_index];
            let mut best_action: usize = 0;
            let mut best_action_score = -99999.9;


            for action_index in 0..self.num_actions {
                let mut total = 0.0;
                for next_state_index in 0..self.num_states {
                    total += self.probabilities[state_index][action_index][next_state_index] *
                        (self.rewards[state_index][action_index][next_state_index]
                        + self.gamma * self.policy[next_state_index] as f32)
                }
                if best_action == 0 || total >= best_action_score {
                    best_action = action_index;
                    best_action_score = total;
                }
            }
            self.policy[state_index] = best_action as f32;
            if self.policy[state_index] != old_action {
                self.is_policy_stable = false;
            }
        }
        return self.is_policy_stable
    }

    fn policy_iteration(&mut self) -> &Vec<f32>{
        loop {
            self.policy_evaluation(0.000001);
            if self.policy_improvement() { break; }
        }
        return &self.policy
    }
}