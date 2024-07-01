use rand::Rng;
use crate::environment::environment::{Environment, State};

pub struct SarsaModel {
    pub q_table: Vec<Vec<f64>>,
    pub alpha: f64,
    pub gamma: f64,
    pub epsilon: f64,
    pub nb_episode: usize,
    pub num_states: usize,
    pub num_actions: usize,
}

impl SarsaModel {
    pub fn new<E: Environment>(env: &E, alpha: f64, gamma:f64, epsilon:f64, nb_episode: usize) -> Box<SarsaModel>{
        let ns = env.all_states().len();
        let na = env.available_actions().len();
        let mut model = Box::new(SarsaModel {
            num_states: ns,
            num_actions: na,
            q_table: vec![vec![0.0; na]; ns],
            alpha,
            gamma,
            epsilon,
            nb_episode
        });
        // Set random Q
        model.init_q(env);
        model
    }

    fn init_q<E: Environment>(&mut self, env: &E) {
        let mut rng = rand::thread_rng();
        for i in 0..self.q_table.len() {
            let si: State = i as State;
            if env.terminal_states().contains(&si) {
                self.q_table[i] = vec![0.0; self.q_table[i].len()];
            } else {
                self.q_table[i] = (0..self.q_table[i].len()).map(|_| rng.gen::<f64>()).collect();
            }
        }
    }

    /*fn iter(&mut self, rand: bool) {
        for _ in 0..self.nb_episode {
            let mut state = if rand == false { 0usize } else {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..self.num_states-1usize)
            };
            let mut action = self.chose_action(state);

            loop {
                let (new_state, reward) = self.do_action(state, action);
                let new_action = self.chose_action(new_state);

                self.q_table[state][action] += self.alpha * (reward + self.gamma
                    * self.q_table[new_state][new_action] - self.q_table[state][action]);

                state = new_state;
                action = new_action;

                if new_state == self.num_states - 1 { break; }
            }
        }
    }
     */

    /*fn chose_action(&mut self, state: usize) -> usize {
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0.0..=1.0);
        return if rand < self.epsilon {
            rng.gen_range(0..=(self.num_actions - 1))
        } else {
            self.q_table[state].iter().enumerate().max_by(|&(_, a), &(_, b) |
            a.partial_cmp(b).unwrap()).map(|(index, _)| index).unwrap()
        }
    }

     */

    /*fn do_action(&mut self, state: usize, action: usize) -> (usize, f32){

    }*/
}