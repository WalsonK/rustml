extern crate rustml;

use rustml::environment::{lineworld, gridworld, tools};
use rustml::dynamic_programming::{policy_iteration, value_iteration};


fn main() {
/*
    //      Line world
    let env = lineworld::LineWorld::new(4, false, 0);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.probabilities);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.rewards);
    let _ = env.display();*/
    
    /*      Grid world
    let env = gridworld::GridWorld::new(3, 5, 1);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.probabilities);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.rewards);
    let _ = env.display();*/


    /*      POLICY ITERATION
    let mut algo = policy_iteration::PolicyIterationModel::new(
        env.all_position,
        env.all_actions,
        env.rewards,
        env.probabilities,
        0.999,
        env.terminal_position
    );
    let best_policy = algo.policy_iteration();
    println!("Policy for policy iter: {:?}", best_policy);*/

    /*      VALUE ITERATION
    let mut val_iter = value_iteration::ValueIterationModel::new(
        env.all_position,
        env.all_actions,
        env.rewards,
        env.probabilities,
        0.999,
        env.terminal_position
    );*/


    val_iter.iteration(0.001);
    println!("Policy for value iter: {:?}", val_iter.policy);

    let mut env = playable_line_world::new(5, false, 2);
    let mut model = MonteCarloControl::new(0.1, 0.9);

    // Entraînement du modèle avec Monte Carlo Control
    model.on_policy_mc_control(&mut *env, 10000, 100);

    // Affichage des résultats après l'entraînement
    println!("Q-values: {:?}", model.q_values);
    println!("Policy: {:?}", model.policy);

    // Tester la politique entraînée sur un état initial
    let state = env.reset();
    let action = model.policy.get(&state).map_or(0, |actions| {
        *actions.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal)).unwrap().0
    });
    env.step(action);
    env.display();

    println!("__________________OFF_Policy_________________");
    let mut env = playable_line_world::new(5, false, 2);
    let mut model = MonteCarloControlOff::new(0.1, 0.9);

    // Entraînement du modèle avec Monte Carlo Control hors politique
    model.off_policy_mc_control(&mut *env, 10000, 100);

    // Affichage des résultats après l'entraînement
    println!("Q-values: {:?}", model.q_values);
    println!("Policy: {:?}", model.policy);

    // Tester la politique entraînée sur un état initial
    let state = env.reset();
    let action = model.policy.get(&state).map_or(0, |actions| {
        *actions.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal)).unwrap().0
    });
    env.step(action);

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
