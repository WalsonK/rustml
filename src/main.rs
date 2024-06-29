use rustml::environment::monty_hall::MontyHall;

fn main() {
    let mut game = MontyHall::new(3);
    println!("{}", game);

    // Example usage
    println!("Available actions: {:?}", game.available_actions());
    let result = game.step(0);
    println!("{}", game);
    println!("Step result: {:?}", result);
    println!(
        "Available actions before remove a door: {:?}",
        game.available_actions()
    );
    let result = game.step(2);
    println!("{}", game);
    println!(
        "Available actions after remove a door: {:?}",
        game.available_actions()
    );
    println!("Step result: {:?}", result);
    println!("Game over: {}", game.is_game_over());
    println!("Current score: {}", game.score());
}

/*mod two_round_rock_paper_scissors;
use crate::two_round_rock_paper_scissors::{Action, Agent, Adversary, Environment};
fn main() {
    let mut env = Environment::new();
    let agent_action_round_1 = env.agent.choose_action();
    let (result_round_1, _) = env.step(agent_action_round_1);
    println!("Round 1: Agent chose {:?}, result: {}", agent_action_round_1, result_round_1);

    let agent_action_round_2 = env.agent.choose_action(); // Agent choisit une action aléatoire pour le deuxième round
    let (result_round_2, done) = env.step(agent_action_round_2); // Passer l'action du deuxième round
    println!("Round 2: Agent chose {:?}, result: {}", agent_action_round_2, result_round_2); // Agent joue la même action que le deuxième round

    if done {
        println!("Game over. Total score: {}", env.agent_score);
    }
}*/
