use rand::Rng;

pub struct SarsaModel {
    pub q_table: Vec<Vec<f32>>,
    pub alpha: f32,
    pub gamma: f32,
    pub epsilon: f32,
    pub nb_episode: usize,
    pub num_states: usize,
    pub num_actions: usize,
}

impl SarsaModel {
    fn new(a: usize, s: usize, al: f32, g:f32, e:f32, ep: usize) -> Box<SarsaModel>{
        let mut model = Box::new(SarsaModel {
            num_states: s,
            num_actions: a,
            q_table: vec![vec![0.0; a]; s],
            alpha: al,
            gamma: g,
            epsilon: e,
            nb_episode: ep
        });
        model
    }

    fn iter(&mut self, rand: bool) {
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

    fn chose_action(&mut self, state: usize) -> usize {
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0.0..=1.0);
        return if rand < self.epsilon {
            rng.gen_range(0..=(self.num_actions - 1))
        } else {
            self.q_table[state].iter().enumerate().max_by(|&(_, a), &(_, b) |
            a.partial_cmp(b).unwrap()).map(|(index, _)| index).unwrap()
        }
    }

    fn do_action(&mut self, state: usize, action: usize) -> (usize, f32){

    }
}