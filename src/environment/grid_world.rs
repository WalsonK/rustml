use rand::Rng;
use crate::environment::environment::{State, Action, Reward, Environment};
use crate::environment::tools;

pub struct GridWorld {
    pub agent_position: State,
    pub lines: usize,
    pub col: usize,
    pub all_position: Vec<i64>,
    pub terminal_position: Vec<State>,
    pub all_actions: Vec<i64>,
    pub probabilities: Vec<Vec<Vec<f64>>>,
    pub rewards: Vec<Vec<Vec<f64>>>,
}

impl GridWorld {
    pub fn new(lines: i64, cols: i64, pos: State) -> Box<GridWorld> {
        let mut positions = Vec::new();
        for i in 0..lines {
            for j in 0..cols {
                positions.push(i * cols + j);
            }
        }
        let mut env = Box::new(GridWorld {
            agent_position: pos,
            lines: lines as usize,
            col: cols as usize,
            all_position: positions,
            terminal_position: vec![0, ((lines - 1) * cols + (cols - 1)) as State],
            all_actions: vec![0, 1, 2, 3, 4],
            probabilities: vec![vec![vec![0.0; (lines * cols) as usize]; 5]; (lines * cols) as usize],
            rewards: vec![vec![vec![0.0; (lines * cols) as usize]; 5]; (lines * cols) as usize],
        });
        env.generate_probabilities();
        env.generate_rewards();
        env
    }

    fn generate_probabilities(&mut self) {
        let num_positions = self.all_position.len();
        let num_actions = self.all_actions.len();
        let begin_position = self.agent_position;
        for position_index in 0..num_positions {
            let current_position = position_index;
            for action_index in 0..num_actions {
                let action = self.all_actions[action_index];
                let available_act = self.available_actions();

                if available_act.contains(&(action as usize)) {
                    self.agent_position = current_position;
                    self.step(action as usize);

                    let next_state = self.state_id() as usize;
                    self.probabilities[position_index][action_index][next_state] = 1.0;

                    self.agent_position = current_position;
                }
            }
        }
        self.agent_position = begin_position;
    }

    fn generate_rewards(&mut self) {
        let num_positions = self.all_position.len();
        let num_actions = self.all_actions.len();

        let current_position = self.agent_position;
        for position_index in 0..num_positions {
            self.agent_position = position_index;
            for action_index in 0..num_actions {
                let action = self.all_actions[action_index];

                if self.available_actions().contains(&(action as usize)) {
                    self.step(action as usize);
                }
                let next_state = if self.state_id() == 0 { 0 } else { self.state_id() };
                let reward = self.score();

                self.rewards[position_index][action_index][next_state] = reward as f64;

                self.agent_position = position_index;
            }
        }
        self.agent_position = current_position;
    }

    fn find_index(grid: &Vec<Vec<i64>>, val: usize) -> (usize, usize) {
        for (i, line) in grid.iter().enumerate() {
            if let Some(j) = line.iter().position(|&x| x == val as i64) {
                return (i, j);
            }
        }
        unreachable!();
    }

    pub fn get_grid(flat_vec: Vec<i64>, col: usize) -> Vec<Vec<i64>> {
        flat_vec.chunks(col).map(|chunk| chunk.to_vec()).collect()
    }

    pub fn get_display_array(&self) -> Vec<Vec<char>> {
        let grid = Self::get_grid(self.all_position.clone(), self.col);
        let mut renderer: Vec<Vec<char>> = Vec::new();
        for line in grid.iter() {
            let mut render_line: Vec<char> = Vec::new();
            for &val in line.iter() {
                if self.agent_position == val as usize {
                    render_line.push('X')
                } else {
                    render_line.push('_')
                }
            }
            renderer.push(render_line);
        }
        for line in renderer.iter() {
            for &val in line.iter() {
                print!("{}", val);
            }
            println!();
        }
        renderer
    }
}

impl Environment for GridWorld {
    fn random_state(&mut self){

    }
    fn transition_probability(&self, state: usize, action: usize, next_state: usize, reward: usize) -> f32{
        0.0
    }

    fn reset(&mut self) -> State {
        self.agent_position = rand::thread_rng().gen_range(1..self.all_position.len());
        self.agent_position as State
    }

    fn step(&mut self, action: Action) -> (State, Reward, bool) {
        if self.is_game_over() {
            return (self.agent_position as State, self.score(), true);
        }

        let grid = GridWorld::get_grid(self.all_position.clone(), self.col);
        match action {
            1 if self.agent_position % self.col != 0 => self.agent_position -= 1,
            2 if self.agent_position % self.col != self.col - 1 => self.agent_position += 1,
            3 => {
                let (line, index) = GridWorld::find_index(&grid, self.agent_position);
                if line + 1 < grid.len() {
                    self.agent_position = grid[line + 1][index] as State;
                }
            }
            4 => {
                let (line, index) = GridWorld::find_index(&grid, self.agent_position);
                if line > 0 {
                    self.agent_position = grid[line - 1][index] as State;
                }
            }
            _ => {}
        }

        let reward = self.score();
        let done = self.is_game_over();
        (self.agent_position as State, reward, done)
    }

    fn available_actions(&self) -> Vec<Action> {
        let mut actions = vec![0];
        if self.agent_position % self.col != 0 {
            actions.push(1);
        }
        if self.agent_position % self.col != self.col - 1 {
            actions.push(2);
        }
        if self.agent_position < self.all_position.len() - self.col {
            actions.push(3);
        }
        if self.agent_position >= self.col {
            actions.push(4);
        }
        if self.terminal_position.contains(&self.agent_position) {
            actions = vec![0];
        }
        actions
    }

    fn all_states(&self) -> Vec<State> {
        self.all_position.iter().map(|&pos| pos as State).collect()
    }

    fn set_state(&mut self, state: State) {
        self.agent_position = state;
    }

    fn display(&self) {
        let grid = GridWorld::get_grid(self.all_position.clone(), self.col);
        for line in &grid {
            for &val in line {
                print!("{}", if val == self.agent_position as i64 { 'X' } else { '_' });
            }
            println!();
        }
    }

    fn state_id(&self) -> State {
        self.agent_position as State
    }

    fn score(&self) -> Reward {
        if self.agent_position == self.terminal_position[0] {
            -1.0
        } else if self.agent_position == self.terminal_position[1] {
            1.0
        } else {
            0.0
        }
    }

    fn is_game_over(&self) -> bool {
        self.terminal_position.contains(&self.agent_position)
    }
    fn all_action(&self) -> Vec<Action> {
        self.all_actions.iter().map(|&action| action as Action).collect()
    }

    fn terminal_states(&self) -> Vec<State> { self.terminal_position.clone() }

    fn is_forbidden(&self, state_or_action: usize) -> bool{
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_grid_world() -> Box<GridWorld>{
        let env = GridWorld::new(3, 5, 1);
        env
    }

    #[test]
    fn test_init() {
        let env = setup_grid_world();
        assert_eq!(env.agent_position, 1);
        assert_eq!(env.col, 5);
        assert_eq!(env.all_position, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
        assert_eq!(env.terminal_position, vec![0, 14]);
    }
    #[test]
    fn test_available_actions() {
        let mut env = setup_grid_world();
        assert_eq!(env.available_actions(), vec![0, 1, 2, 3]);
        env.agent_position = 5;
        assert_eq!(env.available_actions(), vec![0, 2, 3, 4]);
        env.agent_position = 14;
        assert_eq!(env.available_actions(), vec![0]);
    }
    #[test]
    fn test_game_over() {
        let mut env = setup_grid_world();
        assert_eq!(env.is_game_over(), false);
        env.agent_position = 14;
        assert_eq!(env.is_game_over(), true);
    }
    #[test]
    fn test_state() {
        let mut env = setup_grid_world();
        assert_eq!(env.state_id(), 1);
        env.agent_position = 4;
        assert_eq!(env.state_id(), 4);
    }
    #[test]
    fn test_step() {
        let mut env = setup_grid_world();
        env.step(2);
        assert_eq!(env.agent_position, 2);
        env.step(1);
        assert_eq!(env.agent_position, 1);
        env.step(3);
        assert_eq!(env.agent_position, 6);
        env.step(4);
        assert_eq!(env.agent_position, 1);
    }
    #[test]
    fn test_score() {
        let mut env = setup_grid_world();
        env.agent_position = 14;
        assert_eq!(env.score(), 1.0);
        env.agent_position = 0;
        assert_eq!(env.score(), -1.0);
    }
    #[test]
    fn test_display() {
        let env = setup_grid_world();
        let array = env.get_display_array();
        assert_eq!(array, vec![
            vec!['_', 'X', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_']
        ]);
    }
    #[test]
    fn test_reset() {
        let mut env = setup_grid_world();
        env.agent_position = 4;
        env.reset();
        assert_eq!(env.agent_position, 2);
    }
}