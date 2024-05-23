use rand::Rng;

pub struct LineWorld {
    agent_pos: i64,
    all_pos: Vec<i64>,
    go_pos: Vec<i64>
}

impl LineWorld {
    fn new(len: i64, is_rand: bool, pos: i64) -> Box<LineWorld> {
        let env = Box::new(LineWorld {
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

    fn display(&self) -> Vec<char>{
        let mut renderer: Vec<char>= Vec::new();
        for i in self.all_pos[0]..=self.all_pos.len() as i64 {
            if self.agent_pos == i { renderer.push('X') } else {renderer.push('_') }
        }
        let game: String = renderer.iter().collect();
        println!("{}", game);
        renderer
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

#[cfg(test)]
mod init_tests {
    use super::*;

    fn setup_line_world() -> Box<LineWorld>{
        let env = LineWorld::new(4, false, 2);
        env
    }

    #[test]
    fn test_init() {
        let env = setup_line_world();
        assert_eq!(env.agent_pos, 2);
        assert_eq!(env.all_pos, vec![1, 2, 3, 4]);
        assert_eq!(env.go_pos, vec![1, 4]);
    }
    #[test]
    fn test_is_game_over() {
        let mut env = setup_line_world();
        assert_eq!(env.is_game_over(), false);
        env.agent_pos = 4;
        assert_eq!(env.is_game_over(), true);
    }
    #[test]
    fn test_state_id() {
        let mut env = setup_line_world();
        assert_eq!(env.state_id(), 2);
        env.agent_pos = 4;
        assert_eq!(env.state_id(), 4);
    }
    #[test]
    fn test_step() {
        let mut env = setup_line_world();
        for _ in 0..2 {
            env.step(2);
        }
        assert_eq!(env.agent_pos, 4);
    }
    #[test]
    fn test_score() {
        let mut env = setup_line_world();
        env.agent_pos = 4;
        assert_eq!(env.score(), 1.0);
        env.agent_pos = 1;
        assert_eq!(env.score(), -1.0);
    }
    #[test]
    fn test_display() {
        let env = setup_line_world();
        let array = env.display();
        assert_eq!(array, vec!['_', 'X', '_', '_']);
    }
    #[test]
    fn test_reset() {
        let mut env = setup_line_world();
        env.agent_pos = 4;
        env.reset(false, 2);
        assert_eq!(env.agent_pos, 2);
    }
}