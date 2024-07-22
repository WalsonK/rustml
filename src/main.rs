extern crate rustml;

use std::io;
use rustml::environment::{
    line_world, grid_world, playable_monte_hall,
    two_round_rock_paper_scissors, secret_env0dp::SecretEnv0Dp, secret_env1dp::SecretEnv1Dp,
    secret_env2dp::SecretEnv2Dp, secret_env3dp::SecretEnv3Dp,
};
use rustml::environment::tools::{Policy, use_policy_in_game};
use rustml::environment::environment::Environment;
use rustml::environment::environment::Action as ActionType;
use rustml::dynamic_programming::{policy_iteration, value_iteration};
use rustml::td_learning::sarsa;
use rustml::monte_carlo::{monte_carlo_es, monte_carlo_control_struct, monte_carlo_control_struct_off};
use rustml::planning::dyna_q::DynaQModel;
use rustml::planning::dyna_q_plus::DynaQPlusModel;
use rustml::td_learning::q_learning::QLearning;
use rand::Rng;
use std::time::Instant;


fn main() {

    // -------------------------------- ENV -------------------------------------
    /*      Secret Env 0
    let mut env: Box<SecretEnv0Dp> = unsafe { SecretEnv0Dp::new() };
    println!("Env0, action : {:?}, state : {:}", env.all_action(), env.all_states().len());
    env.display();
     */
    /*      Secret Env 1
    let mut env: Box<SecretEnv1Dp> = unsafe { SecretEnv1Dp::new() };
    println!("Env1, action : {:?}, state : {:}",env.all_action(),env.all_states().len());
    env.display();
     */
    /*      Secret Env 2
    let mut env: Box<SecretEnv2Dp> = unsafe { SecretEnv2Dp::new() };
    println!("Env2, action : {:?}, state : {:}",env.all_action(),env.all_states().len());
    env.display();
     */
    /*      Secret Env 3
    let mut env: Box<SecretEnv3Dp> = unsafe { SecretEnv3Dp::new() };
    println!("Env3, action : {:?}, state : {:}",env.all_action(),env.all_states().len());
    env.display();
     */

    //      Line world
    let mut env = line_world::LineWorld::new(4, false, 1);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.probabilities);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.rewards);
    let _ = env.display();
    //

    /*      Grid world
    let mut env = grid_world::GridWorld::new(3, 5, 1);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.probabilities);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.rewards);
    let _ = env.display();
    */

    /*      PLAYABLE MONTY HALL
    let mut env = playable_monte_hall::playable_MontyHall::new(3);
    */

    // two_round_rock_paper_scissors
    //let mut env = two_round_rock_paper_scissors::RPSGame::new();


// -------------------------------- ALGO -------------------------------------

    //     POLICY ITERATION
    let mut model = policy_iteration::PolicyIterationModel::new(
        env.all_position.clone(),
        env.all_actions.clone(),
        env.rewards.clone(),
        env.probabilities.clone(),
        0.999,
        env.terminal_position.clone()
    );
    let start = Instant::now();
    let best_policy = model.policy_iteration();
    let duration = start.elapsed();
    println!("Policy for policy iter: {:?}", best_policy);
    println!("Model trained for : {:?}", duration);
    //println!("final policy: {:?}", model.policy_to_hashmap());
    //model.save_policy("policy_POLICY_ITERATION.json").unwrap();

    //model.load_policy("policy_POLICY_ITERATION.json").unwrap();
    //model.print_policy();
    use_policy_in_game(&mut *env, Policy::Array(best_policy.clone()));
    //


    /*      VALUE ITERATION
    let mut model = value_iteration::ValueIterationModel::new(
        env.all_position.clone() ,
        env.all_actions.clone(),
        env.rewards.clone(),
        env.probabilities.clone(),
        0.999,
        env.terminal_position.clone()
    );
    let start = Instant::now();
    model.iteration(0.01);
    let duration = start.elapsed();
    println!("Policy for value iter: {:?}", model.policy);
    println!("Model trained for : {:?}", duration);
    //model.save_policy("policy_VALUE_ITERATION.json").unwrap();
    //model.load_policy("policy_VALUE_ITERATION.json").unwrap();
    //model.print_policy();
    use_policy_in_game(&mut *env, Policy::Array(model.policy.clone()));

     */


    /*     MONTE CARLO ES
    let mut model = monte_carlo_es::MonteCarloESModel::new(1000, 0.6, 20);
    // Entraînement du modèle avec Monte Carlo ES
    let start = Instant::now();
    model.monte_carlo_es(&mut *env);
    let duration = start.elapsed();
    // Affichage des résultats après l'entraînement pour inspection manuelle
    println!("Q-values: {:?}", model.q_values);
    println!("Policy: {:?}", model.policy);
    println!("Model trained for : {:?}", duration);
    // Tester la politique entraînée sur un état initial
    let state = env.reset();
    let action = model.policy.get(&state).cloned().unwrap_or(0);
    env.step(action);
    env.display();
    use_policy_in_game(&mut *env, Policy::Map(model.policy.clone()));
    model.save_policy("policy_MONTE_CARLO_ES.json").unwrap();
    //let mut model = monte_carlo_es::MonteCarloESModel::new(1000, 0.9, 2);
    //model.load_policy("policy_MONTE_CARLO_ES.json").unwrap();
    */




    /*      MONTE CARLO CONTROL
    let mut model = monte_carlo_control_struct::MonteCarloControl::new(0.1, 0.9);
    // Entraînement du modèle avec Monte Carlo Control
    let start = Instant::now();
    model.on_policy_mc_control(&mut *env, 10000, 100);
    let duration = start.elapsed();
    // Affichage des résultats après l'entraînement
    println!("Q-values: {:?}", model.q_values);
    println!("Policy: {:?}", model.policy);
    println!("Model trained for : {:?}", duration);
    // Tester la politique entraînée sur un état initial
    let state = env.reset();
    let action = model.policy.get(&state).map_or(0, |actions| {
        *actions.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal)).unwrap().0
    });
    env.step(action);
    env.display();
    use_policy_in_game(&mut *env, Policy::Map(model.derived_policy.clone()));
    //model.save_policy("policy_MONTE_CARLO_CONTROL.json").unwrap();
    //model.load_policy("policy_MONTE_CARLO_CONTROL.json").unwrap();
    //println!("Policy  : {:?}", model.derived_policy);

    */


    /*      MONTE CARLO CONTROL OFF POLICY
    let mut model = monte_carlo_control_struct_off::MonteCarloControlOff::new(0.1, 0.9);
    // Entraînement du modèle avec Monte Carlo Control hors politique
    let start = Instant::now();
    model.off_policy_mc_control(&mut *env, 10000, 100);
    let duration = start.elapsed();
    // Affichage des résultats après l'entraînement
    println!("Q-values: {:?}", model.q_values);
    println!("Policy: {:?}", model.policy);
    println!("Model trained for : {:?}", duration);
    // Tester la politique entraînée sur un état initial
    let state = env.reset();
    let action = model.policy.get(&state).map_or(0, |actions| {
        *actions.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal)).unwrap().0
    });
    env.step(action);
    use_policy_in_game(&mut *env, Policy::Map(model.derived_policy.clone()));
    model.save_policy("policy_MONTE_CARLO_CONTROL_OFF.json").unwrap();
    //model.load_policy("policy_MONTE_CARLO_CONTROL_OFF.json").unwrap();
    //println!("Policy  : {:?}", model.derived_policy);*/

    /* SARSA
    let mut model = sarsa::SarsaModel::new(&mut *env, 0.1, 0.9, 0.9, 100);
    //tools::print_matrix(&env.all_position, &env.all_actions, &model.q_table)
    let start = Instant::now();
    let best_policy = model.process_episode(true, &mut *env);
    let duration = start.elapsed();
    println!("Policy for policy iter: {:?}", best_policy);
    println!("Model trained for : {:?}", duration);
    env.reset();

    use_policy_in_game(&mut *env, Policy::Array(best_policy.clone()));*/

    /* Q Learning
    let iterations = 100_000;
    let gamma = 0.6;
    let alpha = 0.7;
    let epsilon = 0.7;

    let mut model = QLearning::new(iterations, gamma, alpha, epsilon);
    let start = Instant::now();
    model.q_learning(&mut *env);
    println!("Q-values: {:?}", model.q_values);
    let policy = model.derive_policy();
    let duration = start.elapsed();
    model.print_policy();
    println!("Model trained for : {:?}", duration);
    use_policy_in_game(&mut *env, Policy::Map(model.policy))
    //model.save_policy("policy_QLearning.json").unwrap();
    //let mut model = QLearning::new(iterations, gamma, alpha, epsilon);
    //model.load_policy("policy_QLearning.json").unwrap();
     */




    /*     DYNQ
    let iterations = 10000;
    let gamma = 0.95;
    let alpha = 0.5;
    let epsilon = 0.8;
    let n = 10;

    let mut model = DynaQModel::new(iterations, gamma, alpha, epsilon, n);
    let start = Instant::now();
    model.dyna_q(&mut *env);
    println!("Q-values: {:?}", model.q_values);
    model.derive_and_assign_policy();
    let duration = start.elapsed();
    model.print_policy();
    println!("Model trained for : {:?}", duration);
    use_policy_in_game(&mut *env, Policy::Map(model.policy.clone()));
    model.save_policy( "policy_DYNQ.json").unwrap();
    //let mut model = DynaQModel::new(iterations, gamma, alpha, epsilon, n);
    //model.load_policy("policy.json").unwrap();*/


    /*     DYNQ+
    // Parameters for DynaQ+ model
    let iterations = 10000;
    let gamma = 0.95;
    let alpha = 0.1;
    let epsilon = 0.1;
    let planning_steps = 10;
    let kappa = 0.001;

    let mut model = DynaQPlusModel::new(iterations, gamma, alpha, epsilon, planning_steps, kappa);
    let start = Instant::now();
    model.dyna_q_plus(&mut *env);
    println!("Q-values: {:?}", model.q_values);
    model.derive_and_assign_policy();
    let duration = start.elapsed();
    model.print_policy();
    println!("Model trained for : {:?}", duration);
    use_policy_in_game(&mut *env, Policy::Map(model.policy.clone()));
    model.save_policy("policy_DYNQ_PLUS.json").unwrap();
    //let mut model = DynaQPlusModel::new(iterations, gamma, alpha, epsilon, planning_steps, kappa);
    //model.load_policy("policy_DYNQ_PLUS.json").unwrap();*/
}