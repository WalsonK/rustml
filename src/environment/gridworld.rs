pub struct GridWorld {
    agent_pos: i64,
    col: usize,
    all_pos: Vec<i64>,
    go_pos: Vec<i64>,
}

impl GridWorld {
    fn new(lines: i64, cols: i64, pos: i64) -> Box<GridWorld>{
        let mut positions: Vec<i64> = Vec::new();
        for i in 0..lines {
            for j in 0..cols {
                let index = i * cols + j;
                positions.push(index);
            }
        }
        let env = Box::new(GridWorld {
            agent_pos: pos,
            col: cols as usize,
            all_pos: positions,
            go_pos: vec![0, (lines - 1) * cols + (cols - 1)]
        });
        env
    }

    fn available_actions(&self) -> Vec<i64>{
        // 0 : Stand / 1 : Left / 2 : Right / 3 : Down / 4 : Up
        let mut actions : Vec<i64> = vec![];
        // Without go position
        let mut playable_pos: Vec<i64> = self.all_pos.clone();
        playable_pos.retain(|x| !self.go_pos.contains(x));

        let first_line: Vec<i64> = playable_pos[0..self.col].to_vec();
        let last_line: Vec<i64> = playable_pos[(playable_pos.len() - self.col)..].to_vec();

        if first_line.contains(&self.agent_pos) { // In first line can't go up
            actions = vec![0, 1, 2, 3];
        } else if last_line.contains(&self.agent_pos) {  // In last line can't go down
            actions = vec![0, 1, 2, 4];
        } else if playable_pos.contains(&self.agent_pos) {
            actions = vec![0, 1, 2, 3, 4];
        }
        // Remove action to go left if at the first column
        if self.agent_pos % self.col as i64 == 0 {
            actions.retain(|&action| action != 1); // Remove Left (1)
        }
        // Remove action to go right if at the last column
        if self.agent_pos % self.col as i64 == (self.col as i64 - 1) {
            actions.retain(|&action| action != 2); // Remove Right (2)
        }

        return actions
    }

    fn is_game_over(&self) -> bool { if self.go_pos.contains(&self.agent_pos) {true} else {false}}

    fn state_id(&self) -> i64 { self.agent_pos }

    fn step(&mut self, action: i64) {
        // Assert
        assert!(!self.is_game_over(), "Game is Over !");
        assert!(self.available_actions().contains(&action), "Action : {action} is not playable !");
        // Generate the Grid
        let grid = get_grid(self.all_pos.clone(), self.col);
        // 0 : Stand / 1 : Left / 2 : Right / 3 : Down / 4 : Up
        if action == 1 { self.agent_pos -= 1 }
        if action == 2 { self.agent_pos += 1 }
        if action == 3 {
            let (line, index) = find_index(&grid, self.agent_pos);
            self.agent_pos = grid[line + 1][index];
        }
        if action == 4 {
            let (index, line) = find_index(&grid, self.agent_pos);
            self.agent_pos = grid[line - 1][index];
        }
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

    fn display(&self) -> Vec<Vec<char>>{
        let grid = get_grid(self.all_pos.clone(), self.col);
        let mut renderer: Vec<Vec<char>> = Vec::new();
        for line in grid.iter() {
            let mut render_line: Vec<char> = Vec::new();
            for &val in line.iter() {
                if self.agent_pos == val {
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

    fn reset(&mut self, pos: i64) { self.agent_pos = pos; }
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
        let env = GridWorld::new(2, 4, 1);
        env
    }

    #[test]
    fn test_init() {
        let env = setup_grid_world();
        assert_eq!(env.agent_pos, 1);
        assert_eq!(env.col, 4);
        assert_eq!(env.all_pos, vec![0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(env.go_pos, vec![0, 7]);
    }
    #[test]
    fn test_available_actions() {
        let mut env = setup_grid_world();
        assert_eq!(env.available_actions(), vec![0, 1, 2, 3]);
        env.agent_pos = 5;
        assert_eq!(env.available_actions(), vec![0, 1, 2, 4]);
    }
    #[test]
    fn test_game_over() {
        let mut env = setup_grid_world();
        assert_eq!(env.is_game_over(), false);
        env.agent_pos = 7;
        assert_eq!(env.is_game_over(), true);
    }
    #[test]
    fn test_state() {
        let mut env = setup_grid_world();
        assert_eq!(env.state_id(), 1);
        env.agent_pos = 4;
        assert_eq!(env.state_id(), 4);
    }
    #[test]
    fn test_step() {
        let mut env = setup_grid_world();
        env.step(2);
        assert_eq!(env.agent_pos, 2);
        env.step(1);
        assert_eq!(env.agent_pos, 1);
        env.step(3);
        assert_eq!(env.agent_pos, 5);
        env.step(4);
        assert_eq!(env.agent_pos, 1);
    }
    #[test]
    fn test_score() {
        let mut env = setup_grid_world();
        env.agent_pos = 7;
        assert_eq!(env.score(), 1.0);
        env.agent_pos = 0;
        assert_eq!(env.score(), -1.0);
    }
    #[test]
    fn test_display() {
        let env = setup_grid_world();
        let array = env.display();
        assert_eq!(array, vec![vec!['_', 'X', '_', '_'], vec!['_', '_', '_', '_']]);
    }
    #[test]
    fn test_reset() {
        let mut env = setup_grid_world();
        env.agent_pos = 4;
        env.reset(2);
        assert_eq!(env.agent_pos, 2);
    }
}