extern crate rustml;

use std::io;
use rustml::environment::{
    line_world, grid_world, tools, playable_monte_hall, secret_env0_dp,
    two_round_rock_paper_scissors, secret_env0dp::SecretEnv0Dp, secret_env1dp::SecretEnv1Dp,
    secret_env2dp::SecretEnv2Dp, secret_env3dp::SecretEnv3Dp
};
use rustml::environment::environment::Environment;
use rustml::environment::environment::Action as ActionType;
use rustml::dynamic_programming::{policy_iteration, value_iteration};
//use rustml::td_learning::sarsa;
use rustml::monte_carlo::{monte_carlo_es, monte_carlo_control_struct, monte_carlo_control_struct_off};
use rustml::planning::dyna_q::DynaQModel;
use rustml::planning::dyna_q_plus::DynaQPlusModel;
use rustml::td_learning::q_learning::QLearning;
use rand::Rng;


fn main() {

    // -------------------------------- ENV -------------------------------------
    // Charge la bibliothèque dynamique spécifique à votre environnement secret
    //let mut env: Box<SecretEnv0Dp> = unsafe { SecretEnv0Dp::new() };
    //println!("Env0, action : {:?}, state : {:}", env.all_action(), env.all_states().len());
    //env.display();
    //let mut env: Box<SecretEnv1Dp> = unsafe { SecretEnv1Dp::new() };
    //println!("Env1, action : {:?}, state : {:}",env.all_action(),env.all_states().len());
    //env.display();
    //let mut env: Box<SecretEnv2Dp> = unsafe { SecretEnv2Dp::new() };
    //println!("Env2, action : {:?}, state : {:}",env.all_action(),env.all_states().len());
    //env.display();
    //let mut env: Box<SecretEnv3Dp> = unsafe { SecretEnv3Dp::new() };
    //println!("Env3, action : {:?}, state : {:}",env.all_action(),env.all_states().len());
    //env.display();

    /*      Line world
    let mut env = line_world::LineWorld::new(4, false, 1);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.probabilities);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.rewards);
    let _ = env.display();
   */

    //      Grid world
    let mut env = grid_world::GridWorld::new(3, 5, 1);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.probabilities);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.rewards);
    let _ = env.display();
    //

    /*      PLAYABLE MONTY HALL
    let mut env = playable_monte_hall::playable_MontyHall::new(3);
    */
    
    // two_round_rock_paper_scissors
    //let mut env = two_round_rock_paper_scissors::RPSGame::new();


// -------------------------------- ALGO -------------------------------------

   /*     POLICY ITERATION
    let mut model = policy_iteration::PolicyIterationModel::new(
        env.all_position,
        env.all_actions,
        env.rewards,
        env.probabilities,
        0.999,
        env.terminal_position
    );
    let best_policy = model.policy_iteration();
    println!("Policy for policy iter: {:?}", best_policy);
    println!("final policy: {:?}", model.policy_to_hashmap());
    model.save_policy("policy_POLICY_ITERATION.json").unwrap();

    model.load_policy("policy_POLICY_ITERATION.json").unwrap();
    model.print_policy();

    */




        /*      VALUE ITERATION
        let mut model = value_iteration::ValueIterationModel::new(
            env.all_position ,
            env.all_actions,
            env.rewards,
            env.probabilities,
            0.999,
            env.terminal_position
        );
        //model.iteration(0.01);
        //println!("Policy for value iter: {:?}", model.policy);
        //model.save_policy("policy_VALUE_ITERATION.json").unwrap();
        model.load_policy("policy_VALUE_ITERATION.json").unwrap();
        model.print_policy();

            */



    /*     MONTE CARLO ES
    let mut model = monte_carlo_es::MonteCarloESModel::new(1000, 0.6, 20);
    // Entraînement du modèle avec Monte Carlo ES
    model.monte_carlo_es(&mut *env);
    // Affichage des résultats après l'entraînement pour inspection manuelle
    println!("Q-values: {:?}", model.q_values);
    println!("Policy: {:?}", model.policy);
    // Tester la politique entraînée sur un état initial
    let state = env.reset();
    let action = model.policy.get(&state).cloned().unwrap_or(0);
    env.step(action);
    env.display();
    model.save_policy("policy_MONTE_CARLO_ES.json").unwrap();
    //let mut model = monte_carlo_es::MonteCarloESModel::new(1000, 0.9, 2);
    //model.load_policy("policy_MONTE_CARLO_ES.json").unwrap();

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
    model.save_policy("policy_MONTE_CARLO_CONTROL.json").unwrap();
    //model.load_policy("policy_MONTE_CARLO_CONTROL.json").unwrap();
    //println!("Policy  : {:?}", model.derived_policy);
    */

    //      MONTE CARLO CONTROL OFF POLICY
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
    env.step(action);
    model.save_policy("policy_MONTE_CARLO_CONTROL_OFF.json").unwrap();
    //model.load_policy("policy_MONTE_CARLO_CONTROL_OFF.json").unwrap();
    //println!("Policy  : {:?}", model.derived_policy);

    /* SARSA
    let mut model = sarsa::SarsaModel::new(&mut *env, 0.1, 0.9, 0.1, 1000);
    //tools::print_matrix(&env.all_position, &env.all_actions, &model.q_table)
    let best_policy = model.process_episode(true, &mut *env);
    println!("Policy for policy iter: {:?}", best_policy);
    //tools::use_policy_in_game(&mut *env, &best_policy);
    */

    /* Q Learning
    let iterations = 100_000;
    let gamma = 0.8;
    let alpha = 0.5;
    let epsilon = 0.9;

    let mut model = QLearning::new(iterations, gamma, alpha, epsilon);
    model.q_learning(&mut *env);
    println!("Q-values: {:?}", model.q_values);
    let policy = model.derive_policy();
    model.print_policy();
    model.save_policy("policy_QLearning.json").unwrap();
    //let mut model = QLearning::new(iterations, gamma, alpha, epsilon);
    //model.load_policy("policy_QLearning.json").unwrap();
    */

    /*     DYNQ
    let iterations = 250000;
    let gamma = 0.95;
    let alpha = 0.5;
    let epsilon = 0.8;
    let n = 10;

    let mut model = DynaQModel::new(iterations, gamma, alpha, epsilon, n);
    model.dyna_q(&mut *env);
    println!("Q-values: {:?}", model.q_values);
    let policy = model.derive_policy();
    model.print_policy();
    model.save_policy( "policy_DYNQ.json").unwrap();
    //let mut model = DynaQModel::new(iterations, gamma, alpha, epsilon, n);
    //model.load_policy("policy.json").unwrap();
    */

    /*     DYNQ+
    // Parameters for DynaQ+ model
    let iterations = 10000;
    let gamma = 0.95;
    let alpha = 0.1;
    let epsilon = 0.1;
    let planning_steps = 10;
    let kappa = 0.001;

    /*let mut model = DynaQPlusModel::new(iterations, gamma, alpha, epsilon, planning_steps, kappa);
    model.dyna_q_plus(&mut *env);
    println!("Q-values: {:?}", model.q_values);
    let policy = model.derive_policy();
    model.print_policy();
    model.save_policy("policy_DYNQ_PLUS.json").unwrap();*/
    //let mut model = DynaQPlusModel::new(iterations, gamma, alpha, epsilon, planning_steps, kappa);
    //model.load_policy("policy_DYNQ_PLUS.json").unwrap();
    */


    /*// Exemple de test de la politique entraînée sur un état initial
    // Boucle de jeu jusqu'à la fin en utilisant le modèle entraîné
    let mut rng = rand::thread_rng();
    //let index = rng.gen_range(0..env.all_position.len());

    env.reset();
    // Exemple d'utilisation de l'environnement
    println!("Initial state:");
    env.display();
    loop {
        let state = env.state_id();
        let action = if let Some(&action) = model.policy.get(&state) {
            action
        } else {
            // Choisir une action aléatoire si aucune politique n'est trouvée pour cet état
            let actions = env.available_actions();
            let index = rng.gen_range(0..actions.len());
            actions[index]
        };

        // Appliquer l'action à l'environnement
        let (new_state, reward, done) = env.step(action);

        println!("Action taken: {}", action);
        println!("State after action:");
        env.display();
        println!("Reward received: {}", reward);
        println!("Game over? {}", done);

        if done {
            println!("Game over. Resetting environment.");
            env.reset();
            break;
        }
    }*/

    let mut rng = rand::thread_rng();
    loop {
        let state = env.state_id();
        let action = if let Some(actions) = model.policy.get(&state) {
            *actions.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal)).unwrap().0
        } else {
            // Choisir une action aléatoire si aucune politique n'est trouvée pour cet état
            let actions = env.available_actions();
            let index = rng.gen_range(0..actions.len());
            actions[index]
        };

        // Appliquer l'action à l'environnement
        let (new_state, reward, done) = env.step(action);

        println!("Action taken: {}", action);
        println!("State after action:");
        env.display();
        println!("Reward received: {}", reward);
        println!("Game over? {}", done);

        if done {
            println!("Game over. Resetting environment.");
            env.reset();
            break;
        }
    }

}
