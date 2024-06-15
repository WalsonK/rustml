use rand::Rng;

pub struct PolicyIteration2Model {
    pub states: Vec<i64>,
    pub actions: Vec<i64>,
    pub rewards: Vec<Vec<Vec<f64>>>,
    pub probabilities: Vec<Vec<Vec<f64>>>,
    pub gamma: f64,
    pub is_policy_stable: bool,
    pub policy: Vec<i64>,
    pub value_function: Vec<f64>
}

impl PolicyIteration2Model {
    pub fn new(s: Vec<i64>, a: Vec<i64>, r: Vec<Vec<Vec<f64>>>, p: Vec<Vec<Vec<f64>>>, g: f64, terminal_state: Vec<i64>) -> Box<PolicyIteration2Model> {
        let mut rng = rand::thread_rng();
        let mut pi_model = Box::new(PolicyIteration2Model {
            states: s.clone(),
            actions: a,
            rewards: r,
            probabilities: p, // vec![vec![vec![vec![0.0; 2]; 2]; 2]; 2]
            is_policy_stable: false,
            gamma: g,
            policy: vec![0; s.len()],
            value_function: (0..s.len()).map(|_| rng.gen::<f64>()).collect()
        });
        for &s in terminal_state.iter() {
            pi_model.value_function[s as usize- 1] = 0.0;
        }
        pi_model
    }

    pub fn policy_evaluation(&mut self, theta: f64){
        loop {
            let mut delta: f64 = 0.0;
            for state in 0..self.states.len() -1 {
                let old_value = self.value_function[state];
                let mut value = 0.0;
                for action in 0..self.actions.len() {
                    for next_state in 0..self.states.len()-1 {
                        value += self.probabilities[state][action][next_state]
                            * (self.rewards[state][action][next_state]
                            + self.gamma * self.policy[next_state] as f64);
                    }
                }
                self.value_function[state] = value;
                delta = delta.max((old_value - self.value_function[state]).abs());
            }
            if delta < theta { break; }
        }
    }

    pub fn policy_improvement(&mut self) -> bool{
        self.is_policy_stable = true;
        for state_index in 0..self.states.len() {
            let old_action = self.policy[state_index];
            let mut best_action: usize = old_action as usize;
            let mut best_action_score = f64::NEG_INFINITY;


            for action_index in 0..self.actions.len() {
                let mut total = 0.0;
                for next_state_index in 0..self.states.len() {
                    total += self.probabilities[state_index][action_index][next_state_index] *
                        (self.rewards[state_index][action_index][next_state_index]
                            + self.gamma * self.policy[next_state_index] as f64)
                }
                if best_action == 0 || total >= best_action_score {
                    best_action = action_index;
                    best_action_score = total;
                }
            }
            self.policy[state_index] = best_action as i64 ;
            if self.policy[state_index] != old_action {
                self.is_policy_stable = false;
            }
        }
        return self.is_policy_stable
    }

    pub fn policy_iteration(&mut self) -> &Vec<i64>{
        loop {
            self.policy_evaluation(0.001);
            if self.policy_improvement() { break; }
        }
        return &self.policy
    }

}