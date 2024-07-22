use rand::Rng;
use crate::environment::environment::{Environment, State, Action, Reward};

pub struct SarsaModel {
    pub q_table: Vec<Vec<f32>>,
    pub alpha: f32,
    pub gamma: f32,
    pub epsilon: f64,
    pub nb_episode: usize,
    pub num_states: usize,
    pub num_actions: usize,
    pub policy: Vec<Action>
}

impl SarsaModel {
    pub fn new(env: &mut dyn Environment, alpha: f32, gamma:f32, epsilon:f64, nb_episode: usize) -> Box<SarsaModel>{
        let ns = env.all_states().len();
        let na = env.available_actions().len();
        let mut model = Box::new(SarsaModel {
            num_states: ns,
            num_actions: na,
            q_table: vec![vec![0.0; na]; ns],
            alpha,
            gamma,
            epsilon,
            nb_episode,
            policy: vec![0; ns]
        });
        // Set random Q
        model.init_q(env);
        model
    }

    // Init Q with 0 for terminal State and Random for other
    fn init_q(&mut self, env: &mut dyn Environment) {
        let mut rng = rand::thread_rng();
        for i in 0..self.q_table.len() {
            let si: State = i as State;
            if env.terminal_states().contains(&si) {
                self.q_table[i] = vec![0.0; self.q_table[i].len()];
            } else {
                self.q_table[i] = (0..self.q_table[i].len()).map(|_| rng.gen::<f32>()).collect();
            }
        }
    }

    pub fn process_episode<E: Environment>(&mut self, rand: bool, env: &mut E) -> Vec<Action>{
        for _ in 0..self.nb_episode {
            let mut state: State = if rand == false { 0usize } else {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..self.num_states-1usize)
            } as State;
            let mut action = self.chose_action(state);

            loop {
                let (new_state, reward) = self.do_action(env, state, action);
                let new_action = self.chose_action(new_state);

                self.q_table[state as usize][action as usize] += self.alpha * (reward + self.gamma
                    * self.q_table[new_state as usize][new_action as usize] - self.q_table[state as usize][action as usize]);

                state = new_state;
                action = new_action;

                if new_state == (self.num_states - 1) as State { break; }
            }
        }

        env.reset();
        // Get best Action from Q for each State
        for s in 0..self.q_table.len() {
            let max_index = self.q_table[s]
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(index, _)| index)
                .unwrap();
            self.policy[s] = max_index as Action;
        }
        self.policy.clone()
    }


    fn chose_action(&mut self, state: State) -> Action {
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0.0..=1.0);
        return if rand < self.epsilon {
            // random
            rng.gen_range(0..=(self.num_actions - 1)) as Action
        } else {
            // best action
            self.q_table[state as usize]
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(index, _)| index)
                .unwrap() as Action
        }
    }

    fn do_action<E: Environment>(&mut self, env: &mut E, state: State, action: Action) -> (State, Reward) {
        env.set_state(state as State);
        let tuple: (State, Reward, bool) = env.step(action as Action);
        return (tuple.0, tuple.1)
    }
}