use rand::Rng;

pub struct LineWorld {
    pub agent_pos: i64,
    pub all_pos: Vec<i64>,
    pub go_pos: Vec<i64>
}

impl LineWorld {
    fn new(len: i64, is_rand: bool, pos: i64) -> Box<LineWorld> {
        let mut env = Box::new(LineWorld {
            agent_pos: if !is_rand {
                pos
            } else {
                let mut rng = rand::thread_rng();
                rng.gen_range(1..len)
            },
            all_pos: (1..=len).collect(),
            go_pos :vec![1, len]
        });
        env
    }

    fn available_actions(&self) -> Vec<i64>{
        let mut playable_pos: Vec<i64> = self.all_pos.clone();
        playable_pos.retain(|x| !self.go_pos.contains(x));
        // 0 : Stand / 1 : Left / 2 : Right
        return if playable_pos.contains(&self.agent_pos) { vec![0, 1, 2] } else { vec![] }
    }

    fn is_game_over(&self) -> bool {
        return if self.go_pos.contains(&self.agent_pos) { true } else { false }
    }

    fn state_id(&self) -> i64{
        return self.agent_pos
    }

    fn step(&mut self, action: i64) {
        assert!(!self.is_game_over(), "Game is Over !");
        assert!(self.available_actions().contains(&action), "Action : {action} is not playable !");
        if action == 1 { self.agent_pos -= 1 }
        if action == 2 { self.agent_pos += 1 }
    }

    fn score(&self) -> f64 {
        let mut score: f64 = 0.0;
        if self.agent_pos == self.go_pos[0] {
            score = -1.0
        }
        if self.agent_pos == self.go_pos[1] {
            score = 1.0;
        }
        score
    }

    fn display(&self) {
        for i in 0..=self.all_pos.len() as i64 {
            println!("{}", if self.agent_pos == i {'X'} else {'_'} );
        }
    }

    fn reset(&mut self, is_rand: bool, pos: i64) {
        self.agent_pos = if !is_rand {
            pos
        } else {
            let mut rng = rand::thread_rng();
            rng.gen_range(1..self.all_pos[self.all_pos.len() - 1])
        }
    }
}