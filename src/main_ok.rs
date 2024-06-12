extern crate rustml;

use rustml::environment::lineworld;
use rustml::dynamic_programming::policy_iteration;

fn main() {
    let env = lineworld::LineWorld::new(6, false, 0);
    let disp = env.display();
    println!("{:?}", disp);

    let state = env.state_id() as usize;
    let action = env.available_actions();
    let reward = env.score() as usize;

    let mut pi = policy_iteration::PolicyIterationModel::new(
        state,
        action.len(),
        reward,
        vec![vec![vec![0.0f32; reward]; action.len()]; state],
        0.99
    );
    let mut policy = pi.policy_iteration();
}