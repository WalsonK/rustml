extern crate rustml;

use rustml::environment::{line_world, grid_world, tools, playable_monte_hall, secret_env0_dp,two_round_rock_paper_scissors};
use rustml::environment::environment::Environment;
use rustml::dynamic_programming::{policy_iteration, value_iteration};
use rustml::td_learning::sarsa;
use rustml::monte_carlo::{monte_carlo_es, monte_carlo_control_struct, monte_carlo_control_struct_off};
use rustml::planning::dyna_q::DynaQModel;
use rustml::planning::dyna_q_plus::DynaQPlusModel;


fn main() {

    // -------------------------------- ENV -------------------------------------
    
    /* SECRET ENV 0 DP
    let mut env = secret_env0_dp::SecretEnv0Dp::new();
    println!("s : {}, a: {}, r: {}", env.num_states, env.num_actions, env.num_rewards);
    println!("matrix : {:?}", env.load_rewards())
    */

/*    //      Line world
    let mut env = line_world::LineWorld::new(4, false, 1);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.probabilities);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.rewards);
    let _ = env.display();
    //
*/
    /*      Grid world
    let mut env = grid_world::GridWorld::new(3, 5, 1);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.probabilities);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.rewards);
    let _ = env.display();
    */

    // two_round_rock_paper_scissors
    let mut env = two_round_rock_paper_scissors::RPSGame::new();


// -------------------------------- ALGO -------------------------------------

   //     POLICY ITERATION
    let mut algo = policy_iteration::PolicyIterationModel::new(
        env.all_position,
        env.all_actions,
        env.rewards,
        env.probabilities,
        0.999,
        env.terminal_position
    );
    let best_policy = algo.policy_iteration();
    println!("Policy for policy iter: {:?}", best_policy);


    /*      VALUE ITERATION
    let mut val_iter = value_iteration::ValueIterationModel::new(
        env.all_position,
        env.all_actions,
        env.rewards,
        env.probabilities,
        0.999,
        env.terminal_position
    );
    val_iter.iteration(0.001);
    println!("Policy for value iter: {:?}", val_iter.policy);
    */


/*
    //     MONTE CARLO ES
    let mut model = monte_carlo_es::MonteCarloESModel::new(10000, 0.9, 2);
    // Entraînement du modèle avec Monte Carlo ES
    model.monte_carlo_es(&mut *env);
    // Affichage des résultats après l'entraînement pour inspection manuelle
    println!("Q-values: {:?}", model.q_values);
    println!("Policy: {:?}", model.policy);
    // Tester la politique entraînée sur un état initial
    let state = env.reset(true, 1);
    let action = model.policy.get(&state).cloned().unwrap_or(0);
    env.step(action);
    env.display();
*/

    /*      MONTE CARLO CONTROL
    let mut model = monte_carlo_control_struct::MonteCarloControl::new(0.1, 0.9);
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
    */

    /*      MONTE CARLO CONTROL OFF POLICY
    let mut model = monte_carlo_control_struct_off::MonteCarloControlOff::new(0.1, 0.9);
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
    env.step(action);*/

/*    // SARSA
    let mut model = sarsa::SarsaModel::new(&mut *env, 0.1, 0.9, 0.1, 1000);
    //tools::print_matrix(&env.all_position, &env.all_actions, &model.q_table)
    let best_policy = model.process_episode(true, &mut *env);
    println!("Policy for policy iter: {:?}", best_policy);
    //tools::use_policy_in_game(&mut *env, &best_policy);
*/
    /*     DYNQ
    let iterations = 10000;
    let gamma = 0.95;
    let alpha = 0.1;
    let epsilon = 0.1;
    let n = 100;

    let mut dyna_q_model = DynaQModel::new(iterations, gamma, alpha, epsilon, n);
    dyna_q_model.dyna_q(&mut *env);
    println!("Q-values: {:?}", dyna_q_model.q_values);
    let policy = dyna_q_model.derive_policy();
    dyna_q_model.print_policy(&policy);
    */

    /*     DYNQ+
    // Parameters for DynaQ+ model
    let iterations = 100000;
    let gamma = 0.95;
    let alpha = 0.1;
    let epsilon = 0.1;
    let planning_steps = 100;
    let kappa = 0.001;

    let mut dyna_q_model = DynaQPlusModel::new(iterations, gamma, alpha, epsilon, planning_steps, kappa);
    dyna_q_model.dyna_q_plus(&mut *env);
    println!("Q-values: {:?}", dyna_q_model.q_values);
    let policy = dyna_q_model.derive_policy();
    dyna_q_model.print_policy(&policy);
    */

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
