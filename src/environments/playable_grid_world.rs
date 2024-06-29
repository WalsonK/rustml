extern crate rand;
use rand::Rng;
use crate::environments::environment::{State, Action, Reward, Environment};

pub struct playable_GridWorld {
    agent_pos: i64,
    col: usize,
    all_pos: Vec<i64>,

    go_pos: Vec<i64>,
}

impl playable_GridWorld {
    pub fn new(lines: i64, cols: i64, pos: i64) -> Box<playable_GridWorld> {
        let mut positions = Vec::new();
        for i in 0..lines {
            for j in 0..cols {
                positions.push(i * cols + j);
            }
        }
        let env = Box::new(playable_GridWorld {
            agent_pos: pos,
            col: cols as usize,
            all_pos: positions,
            go_pos: vec![0, (lines - 1) * cols + (cols - 1)],
        });
        env
    }

    fn find_index(grid: &Vec<Vec<i64>>, val: i64) -> (usize, usize) {
        for (i, line) in grid.iter().enumerate() {
            if let Some(j) = line.iter().position(|&x| x == val) {
                return (i, j);
            }
        }
        unreachable!();
    }

    fn get_grid(flat_vec: Vec<i64>, col: usize) -> Vec<Vec<i64>> {
        flat_vec.chunks(col).map(|chunk| chunk.to_vec()).collect()
    }

}

impl Environment for playable_GridWorld {
    fn reset(&mut self) -> State {
        self.agent_pos = rand::thread_rng().gen_range(0..self.all_pos.len() as i64);
        self.agent_pos
    }

    fn step(&mut self, action: Action) -> (State, Reward, bool) {
        if self.is_game_over() {
            return (self.agent_pos, self.score(), true);
        }

        let grid = playable_GridWorld::get_grid(self.all_pos.clone(), self.col);
        match action {
            1 => if self.agent_pos % self.col as i64 != 0 { self.agent_pos -= 1; },
            2 => if self.agent_pos % self.col as i64 != self.col as i64 - 1 { self.agent_pos += 1; },
            3 => {
                let (line, index) = playable_GridWorld::find_index(&grid, self.agent_pos);
                if line + 1 < grid.len() { self.agent_pos = grid[line + 1][index]; }
            },
            4 => {
                let (line, index) = playable_GridWorld::find_index(&grid, self.agent_pos);
                if line > 0 { self.agent_pos = grid[line - 1][index]; }
            },
            _ => {},        // 0 : Stand / 1 : Left / 2 : Right / 3 : Down / 4 : Up

        }

        let reward = self.score();
        let done = self.is_game_over();
        (self.agent_pos, reward, done)
    }

    fn available_actions(&self) -> Vec<Action> {
        let mut actions = vec![0];
        let col_len = self.col as i64;

        if self.agent_pos % col_len != 0 { actions.push(1); }
        if self.agent_pos % col_len != col_len - 1 { actions.push(2); }
        if self.agent_pos < self.all_pos.len() as i64 - col_len { actions.push(3); }
        if self.agent_pos >= col_len { actions.push(4); }

        actions
    }

    fn all_states(&self) -> Vec<State> {
        self.all_pos.clone()
    }

    fn set_state(&mut self, state: State) {
        self.agent_pos = state;
    }

    fn display(&self) {
        let grid = playable_GridWorld::get_grid(self.all_pos.clone(), self.col);
        for line in &grid {
            for &val in line {
                print!("{}", if val == self.agent_pos { 'X' } else { '_' });
            }
            println!();
        }
    }

    fn state_id(&self) -> State {
        self.agent_pos
    }

    fn score(&self) -> Reward {
        if self.agent_pos == self.go_pos[0] {
            -1.0
        } else if self.agent_pos == self.go_pos[1] {
            1.0
        } else {
            0.0
        }
    }
    fn is_game_over(&self) -> bool {
        self.go_pos.contains(&self.agent_pos)
    }
}
