extern crate rustml;

use rand::Rng;
use rustml::environment::{line_world, grid_world, tools, playable_line_world, playable_grid_world, playable_monte_hall};
use rustml::environment::environment::{Action, Environment};
use rustml::dynamic_programming::{policy_iteration, value_iteration};
//use rustml::environment::env0::env0;
use rustml::monte_carlo::{monte_carlo_es, monte_carlo_control_struct, monte_carlo_control_struct_off};

use rustml::environment::SecretEnv0Dp::SecretEnv0Dp;
use rustml::environment::SecretEnv1Dp::SecretEnv1Dp;
use rustml::environment::SecretEnv2Dp::SecretEnv2Dp;
use rustml::environment::SecretEnv3Dp::SecretEnv3Dp;
use rustml::planning::{dyna_q,dyna_q_plus};
use rustml::td_learning::{q_learning,sarsa};
use rustml::td_learning::q_learning::QLearning;


/*
fn main() {
    /*let mut env = unsafe { env0::new() };

    // Exemple d'utilisation
    let initial_state = env.reset();
    println!("Initial state ID: {}", initial_state);

    // Boucle de jeu hypothétique
    while !env.is_game_over() {
        println!("azul");
        let actions = env.available_actions();
        println!("Available actions: {:?}", actions);

        // Choix d'une action aléatoire
        let action = actions[rand::thread_rng().gen_range(0..actions.len())];
        println!("Performing action: {}", action);

        let (new_state, reward, game_over) = env.step(action);
        println!("New state ID: {}, Reward: {}, Game over: {}", new_state, reward, game_over);

        env.display();
    }

    println!("Final score: {}", env.score());
    //    Line world
    let env = lineworld::LineWorld::new(4, false, 1);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.probabilities);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.rewards);
    let _ = env.display();

*/
    //      Playable Line world
    let mut env = playable_line_world::playable_line_world::new(5, false, 2);
     //

    /*     Grid world
    let env = gridworld::GridWorld::new(3, 5, 1);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.probabilities);
    //tools::print_matrix(&env.all_position, &env.all_actions, &env.rewards);
    let _ = env.display();*/

    /*      Playable Grid world
    let mut env = playable_grid_world::playable_GridWorld::new(3,5,1);
    */

    /*      PLAYABLE MONTY HALL
    let mut env = playable_monte_hall::playable_MontyHall::new(3);
    */


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

    /*    VALUE ITERATION
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

    /*      MONTE CARLO ES
    let mut model = monte_carlo_es::MonteCarloESModel::new(10000, 0.9, 2);
    // Entraînement du modèle avec Monte Carlo ES
    model.monte_carlo_es(&mut *env);
    // Affichage des résultats après l'entraînement pour inspection manuelle
    println!("Q-values: {:?}", model.q_values);
    println!("Policy: {:?}", model.policy);
    // Tester la politique entraînée sur un état initial
    let state = env.reset();
    let action = model.policy.get(&state).cloned().unwrap_or(0);
    env.step(action);
    env.display();*/

    //      MONTE CARLO CONTROL
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
    env.display(); //

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

}
*/



mod environment;

fn main() {
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
    let mut env = line_world::LineWorld::new(5, false, 2);
    //let mut env = grid_world::GridWorld::new(3, 5, 1);
    //println!("{:?}",env.all_action());


    /*Exemple de boucle de jeu ou d'interaction avec l'environnement
    for _ in 0..100 {
        let actions = env.available_actions();
        println!("Available actions: {:?}", actions);
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..actions.len());
        let action = actions[index]; // Exemple : choisir la première action disponible

        let (state, reward, done) = env.step(action);

        println!("Action taken: {}", action);
        println!("State after action:");
        env.display();
        println!("Reward received: {}", reward);
        println!("Game over? {}", done);

        if done {
            println!("Game over. Resetting environment.");
            let initial_state = env.reset();
            println!("Environment reset. Initial state:");
            env.display();
        }
    }*/

    //let mut model = monte_carlo_es::MonteCarloESModel::new(50000, 0.9, 1000);
    //let mut model = monte_carlo_control_struct::MonteCarloControl::new(0.1, 0.9);

    //let mut model = dyna_q::DynaQModel::new(200,0.9, 0.5, 0.3, 10);
    //let mut model = monte_carlo_control_struct_off::MonteCarloControlOff::new(0.1, 0.9);
    //model.off_policy_mc_control(&mut *env, 10000, 100);
    //model.dyna_q(&mut *env);
    // Entraînement du modèle avec Monte Carlo Control hors politique
    //model.on_policy_mc_control(&mut *env, 10000, 50);
    //model.monte_carlo_es(&mut *env);
    /*let mut model = monte_carlo_control_struct::MonteCarloControl::new(0.1, 0.9);
    // Entraînement du modèle avec Monte Carlo Control
    model.on_policy_mc_control(&mut *env, 10000, 100);*/
    // Q Learning
    let iterations = 10000;
    let gamma = 0.8;
    let alpha = 0.6;
    let epsilon = 0.9;
    let mut q_learning_model = QLearning::new(iterations, gamma, alpha, epsilon);
    q_learning_model.q_learning(&mut *env);

    println!("Q-values: {:?}", q_learning_model.q_values);
    let policy = q_learning_model.derive_policy();
    q_learning_model.print_policy(&policy);
    // Affichage des résultats après l'entraînement pour inspection manuelle*/

}
/*

    //println!("Q-values: {:?}", model.q_values);
    //let policy = model.derive_policy();
    //println!("Policy: {:?}", model.policy);

    //println!("Policy: {:?}", model.print_policy(&policy));

    // Exemple de test de la politique entraînée sur un état initial
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
    }
    /*loop {
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
    }*/
    }*/