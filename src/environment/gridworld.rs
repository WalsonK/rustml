use crate::environment::tools;
pub struct GridWorld {
    pub agent_position: i64,
    pub lines: usize,
    pub col: usize,
    pub all_position: Vec<i64>,
    pub terminal_position: Vec<i64>,
    pub all_actions: Vec<i64>,
    pub probabilities: Vec<Vec<Vec<f64>>>,
    pub rewards: Vec<Vec<Vec<f64>>>
}

impl GridWorld {
    pub fn new(lines: i64, cols: i64, pos: i64) -> Box<GridWorld>{
        let mut positions: Vec<i64> = Vec::new();
        for i in 0..lines {
            for j in 0..cols {
                let index = i * cols + j;
                positions.push(index);
            }
        }
        let mut env = Box::new(GridWorld {
            agent_position: pos,
            lines: lines as usize,
            col: cols as usize,
            all_position: positions,
            terminal_position: vec![0, (lines - 1) * cols + (cols - 1)],
            // 0 : Stand / 1 : Left / 2 : Right / 3 : Down / 4 : Up
            all_actions: vec![0, 1, 2, 3, 4],
            probabilities: vec![vec![vec![0.0; (lines*cols) as usize];5]; (lines*cols) as usize],
            rewards: vec![vec![vec![0.0; (lines*cols) as usize];5]; (lines*cols) as usize]
        });
        env.generate_probabilities();
        env.generate_rewards();
        env
    }

    fn generate_probabilities(&mut self) {
        let num_positions = self.all_position.len();
        let num_actions = self.all_actions.len();
        let begin_position = self.agent_position;
        for position_index in 1..num_positions {
            let current_position = position_index as i64;
            for action_index in 0..num_actions {
                let action = self.all_actions[action_index];
                let available_act = self.available_actions();

                if available_act.contains(&action) {
                    self.agent_position = current_position;
                    self.step(action);

                    let next_state = self.state_id() as usize;
                    self.probabilities[position_index][action_index][next_state] = 1.0;

                    // Remettre l'agent à la position initiale pour le prochain essai
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
            self.agent_position = position_index as i64;
            for action_index in 0..num_actions {
                let action = self.all_actions[action_index];

                if self.available_actions().contains(&action) { self.step(action); }
                let next_state = if self.state_id() == 0 { 0 } else { self.state_id()};
                let reward = self.score();

                self.rewards[position_index][action_index][next_state as usize] = reward;

                self.agent_position = position_index as i64;
            }
        }
        self.agent_position = current_position;
    }


    pub fn available_actions(&self) -> Vec<i64>{
        // 0 : Stand / 1 : Left / 2 : Right / 3 : Down / 4 : Up
        let mut actions : Vec<i64> = vec![];
        // Without go position
        let mut playable_pos: Vec<i64> = self.all_position.clone();
        playable_pos.retain(|x| !self.terminal_position.contains(x));

        let first_line: Vec<i64> = playable_pos.iter().cloned().filter(|&pos| pos < self.col as i64).collect();
        let last_line: Vec<i64> = playable_pos.iter().cloned().filter(|&pos| pos >= ((self.lines - 1) * self.col) as i64).collect();

        if first_line.contains(&self.agent_position) { // In first line can't go up
            actions = vec![0, 1, 2, 3];
        } else if last_line.contains(&self.agent_position) {  // In last line can't go down
            actions = vec![0, 1, 2, 4];
        } else if playable_pos.contains(&self.agent_position) {
            actions = vec![0, 1, 2, 3, 4];
        }
        // Remove action to go left if at the first column
        if self.agent_position % self.col as i64 == 0 {
            actions.retain(|&action| action != 1); // Remove Left (1)
        }
        // Remove action to go right if at the last column
        if self.agent_position % self.col as i64 == (self.col as i64 - 1) {
            actions.retain(|&action| action != 2); // Remove Right (2)
        }

        // Action 0 for final state
        if self.terminal_position.contains(&self.agent_position){
            actions = vec![0];
        }

        return actions
    }

    pub fn is_game_over(&self) -> bool { if self.terminal_position.contains(&self.agent_position) {true} else {false}}

    pub fn state_id(&self) -> i64 { self.agent_position }

    pub fn step(&mut self, action: i64) {
        // assert!(!self.is_game_over(), "Game is Over !");
        assert!(self.available_actions().contains(&action), "Action : {action} is not playable !");
        // Generate the Grid
        let grid = get_grid(self.all_position.clone(), self.col);
        // 0 : Stand / 1 : Left / 2 : Right / 3 : Down / 4 : Up
        if action == 1 { self.agent_position -= 1 }
        if action == 2 { self.agent_position += 1 }
        if action == 3 {
            let (line, index) = find_index(&grid, self.agent_position);
            self.agent_position = grid[line + 1][index];
        }
        if action == 4 {
            let (line, index) = find_index(&grid, self.agent_position);
            self.agent_position = grid[line - 1][index];
        }
    }

    pub fn score(&self) -> f64 {
        tools::score(self.agent_position, &self.terminal_position)
    }

    pub fn display(&self) -> Vec<Vec<char>>{
        let grid = get_grid(self.all_position.clone(), self.col);
        let mut renderer: Vec<Vec<char>> = Vec::new();
        for line in grid.iter() {
            let mut render_line: Vec<char> = Vec::new();
            for &val in line.iter() {
                if self.agent_position == val {
                    render_line.push('X')
                }else {
                    render_line.push('_')
                }
            }
            renderer.push(render_line);
        }
        // Display in println
        for line in renderer.iter() {
            for &val in line.iter() {
                print!("{}", val);
            }
            println!();
        }
        renderer
    }

    pub fn reset(&mut self, pos: i64) { self.agent_position = pos; }
}

fn find_index(grid: &Vec<Vec<i64>>, val: i64) -> (usize, usize) {
    for (i, line) in grid.iter().enumerate() {
        if let Some(j) = line.iter().position(|&x| x == val) {return (i, j)}
    }
    unreachable!();
}
fn get_grid(flat_vec: Vec<i64>, col: usize) -> Vec<Vec<i64>>{
    let mut grid: Vec<Vec<i64>> = Vec::new();
    let lines = flat_vec.len() / col;
    for i in 0..lines {
        let start = i * col;
        let end = start + col;
        let line: Vec<i64> = flat_vec[start..end].to_vec();
        grid.push(line);
    }
    grid
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
        let array = env.display();
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
        env.reset(2);
        assert_eq!(env.agent_position, 2);
    }
}