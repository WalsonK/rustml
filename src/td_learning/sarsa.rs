use std::collections::HashMap;
use std::fs::File;
use std::{f32, io};
use rand::Rng;
use rand::seq::SliceRandom;
use crate::environment::environment::{Environment, State, Action, Reward};

pub struct SarsaModel {
    pub alpha: f32,
    pub gamma: f32,
    pub epsilon: f64,
    pub nb_episode: usize,
    pub num_states: usize,
    pub num_actions: usize,
    pub policy: Vec<Action>,
    pub q_values: HashMap<(State, Action), Reward>
}

impl SarsaModel {
    pub fn new(env: &mut dyn Environment, alpha: f32, gamma:f32, epsilon:f64, nb_episode: usize) -> Box<SarsaModel>{
        let ns = env.all_states().len();
        let na = env.available_actions().len();
        let mut model = Box::new(SarsaModel {
            num_states: ns,
            num_actions: na,
            alpha,
            gamma,
            epsilon,
            nb_episode,
            policy: vec![0; ns],
            q_values: HashMap::new()
        });
        model
    }

    pub fn process_episode(&mut self, env: &mut dyn Environment) -> Vec<Action>{
        for num_episode in 0..self.nb_episode {
            println!("Episode n°{} start", num_episode);
            env.reset();
            /*let mut state: State = if rand == false { 0usize } else {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..self.num_states-1usize)
            } as State;*/
            let mut action = self.chose_action(env, env.state_id());

            let mut count = 0;
            loop {
                let state = env.state_id();

                let (new_state, reward) = self.do_action(env, action);
                let new_action = self.chose_action(env, env.state_id());

                let q_value_key = (state, action);
                let next_q_value_key = (new_state, new_action);

                let q_value = *self.q_values.entry(q_value_key).or_insert(0.0);
                let next_q_value = *self.q_values.entry(next_q_value_key).or_insert(0.0);

                let updated_q_value = q_value + self.alpha * (reward + self.gamma * next_q_value - q_value);

                self.q_values.insert(q_value_key, updated_q_value);

                action = new_action;

                if new_state == state { count += 1; }
                else { count = 0; }

                if (new_state == (self.num_states - 1) as State || count > 20) { break; }
            }
            println!("Episode n°{} end", num_episode);
        }
        self.derive_policy()
    }

    fn derive_policy(&mut self) -> Vec<Action> {
        for s in 0..self.num_states {
            let mut best_action = 0;
            let mut max_value = f32::NEG_INFINITY;

            for (&(state, action), &q_value) in &self.q_values {
                if state == s {
                    if q_value > max_value {
                        max_value = q_value;
                        best_action = action;
                    }
                }
            }
            self.policy[s] = best_action as Action;
        }
        self.policy.clone()
    }


    fn chose_action(&mut self, env: &dyn Environment, state: State) -> Action {
        let available_actions = env.available_actions();

        let mut best_action = 0 as Action;
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0.0..=1.0);
        best_action = if rand < self.epsilon {
            // random
            *available_actions.choose(&mut rng).unwrap()
            //rng.gen_range(0..self.num_actions)
        } else {
            // best action
            let action = self.q_values.iter()
                .filter_map(|(&(s, action), &q_value)| {
                    if s == state {
                        Some((q_value, action))
                    } else {
                        None
                    }
                })
                .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or_else(|| a.0.total_cmp(&b.0))) // Trouver la valeur maximale
                .map(|(_, action)| action) // Extraire l'action
                .unwrap_or( *available_actions.choose(&mut rng).unwrap() ) as Action;
            action
        };
        best_action
    }

    fn do_action(&mut self, env: &mut dyn Environment, action: Action) -> (State, Reward) {
        println!("\nstate :{}, action : {}", env.state_id(), action);
        let tuple: (State, Reward, bool) = env.step(action as Action);
        env.display();
        return (tuple.0, tuple.1)
    }

    fn policy_to_hashmap(&self) -> HashMap<usize, usize> { // Changed to usize
        let mut policy_map = HashMap::new();
        for (state, &action) in self.policy.iter().enumerate() {
            policy_map.insert(state, action);
        }
        policy_map
    }
    pub fn save_policy(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, &self.policy_to_hashmap())?;
        Ok(())
    }
    pub fn load_policy(&mut self, filename: &str) -> io::Result<Vec<usize>> { // Changed to usize
        let file = File::open(filename)?;
        let map: HashMap<usize, usize> = serde_json::from_reader(file)?;
        let max_key = match map.keys().max() {
            Some(&key) => key,
            None => 0,
        };
        let mut vec = vec![0; max_key + 1];
        for (key, value) in map {
            vec[key] = value;
        }
        self.policy = vec.clone();
        Ok(vec)
    }
}