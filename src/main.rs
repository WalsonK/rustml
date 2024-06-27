mod dynamic_programming {
    pub mod monte_carlo;
}

mod environments;

use dynamic_programming::monte_carlo::monte_carlo_es::MonteCarloESModel;
use dynamic_programming::monte_carlo::monte_carlo_control_struct::MonteCarloControl;
use dynamic_programming::monte_carlo::monte_carlo_control_struct_off::MonteCarloControlOff;

use environments::line_world::LineWorld;
use environments::grid_world::GridWorld;
use environments::environment::Environment;
use crate::environments::monte_hall::MontyHall;

fn main() {
/*
    let mut env = MontyHall::new(3);
    let mut model = MonteCarloESModel::new(10000, 0.9, 2);

    model.monte_carlo_es(&mut *env);

    // Affichage des résultats après l'entraînement pour inspection manuelle
    println!("Q-values: {:?}", model.q_values);
    println!("Policy: {:?}", model.policy);

    let state = env.reset();
    let action = model.policy.get(&state).cloned().unwrap_or(0);
    env.step(action);
    env.display();


    // Test with LineWorld
    println!("Testing Monte Carlo ES with LineWorld:");
    let mut line_world_env = LineWorld::new(4, false, 2);

    println!("{:?}", line_world_env.all_states());
    line_world_env.display(); // Display the initial state of the environment

    let mut mc_es_model_line_world = MonteCarloESModel::new(10000, 0.99, 100); // Add a step limit per episode
    println!("Starting Monte Carlo ES for LineWorld...");
    mc_es_model_line_world.monte_carlo_es(line_world_env.as_mut());

    println!("Monte Carlo ES completed for LineWorld. Policy:");
    for (state, action) in mc_es_model_line_world.policy.iter() {
        println!("State: {}, Best Action: {}", state, action);
    }
    println!("Q-values: {:?}", mc_es_model_line_world.q_values);
    println!("Policy: {:?}", mc_es_model_line_world.policy);

    // Uncomment to test with GridWorld

    println!("Testing Monte Carlo ES with GridWorld:");
    let mut grid_world_env = GridWorld::new(5, 5, 5);
    grid_world_env.display(); // Display the initial state of the environment

    let mut mc_es_model_grid_world = MonteCarloESModel::new(1000, 0.999, 100); // Add a step limit per episode
    println!("Starting Monte Carlo ES for GridWorld...");
    mc_es_model_grid_world.monte_carlo_es(grid_world_env.as_mut());

    println!("Monte Carlo ES completed for GridWorld. Policy:");
    for (state, action) in mc_es_model_grid_world.policy.iter() {
        println!("State: {}, Best Action: {}", state, action);
    }

    println!("Testing Monte Carlo ES with LineWorld:");
    let mut line_world_env = LineWorld::new(5, false, 2);

    println!("{:?}", line_world_env.all_states());
    line_world_env.display(); // Display initial state of the environment

    let mut mc_control = MonteCarloControl::new(0.1, 0.999); // Set epsilon and gamma
    mc_control.on_policy_mc_control(line_world_env.as_mut(), 10000, 1000); // Set num_episodes and max_steps

    println!("Monte Carlo ES completed for LineWorld. Policy:");
    for (state, actions) in mc_control.policy.iter() {
        for (action, prob) in actions.iter() {
            println!("State: {}, Action: {}, Probability: {}", state, action, prob);
        }
    }

    // Test with GridWorld
    println!("Testing Monte Carlo ES with GridWorld:");
    let mut grid_world_env = GridWorld::new(3, 5, 1);
    grid_world_env.display(); // Display initial state of the environment

    let mut mc_control_grid = MonteCarloESModel::new(10000, 0.9, 2); // Set epsilon and gamma
    mc_control_grid.monte_carlo_es(&mut *grid_world_env);

    println!("Monte Carlo ES completed for GridWorld. Policy:");
    for (state, action) in mc_control_grid.policy.iter() {
        println!("State: {}, Best Action: {}", state, action);
    }
    println!("Q-values: {:?}", mc_control_grid.q_values);
    println!("Policy: {:?}", mc_control_grid.policy);

    let mut env = MontyHall::new(3);
    let mut model = MonteCarloControl::new(0.1, 0.9);

    model.on_policy_mc_control(&mut *env, 10000, 2);

    // Affichage des résultats après l'entraînement pour inspection manuelle
    println!("Q-values: {:?}", model.q_values);
    println!("Policy: {:?}", model.policy);

    let state = env.reset();
    let action = model.policy.get(&state).map_or(0, |actions| {
        *actions.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal)).unwrap().0
    });
    env.step(action);
    env.display();

    let mut env =LineWorld::new(5, false, 2);
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
    env.display();*/
    let mut env = LineWorld::new(5, false, 2);
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
