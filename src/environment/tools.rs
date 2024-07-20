use crate::environment::environment::{Action, Environment,State};
use libloading::Library;

pub fn print_matrix(all_position: &Vec<i64>, all_actions: &Vec<i64>, matrix: &Vec<Vec<Vec<f64>>>) {
    println!("Matrix:");
    for (pos_idx, position_rewards) in matrix.iter().enumerate() {
        println!("Position {}:", all_position[pos_idx]);
        for (action_idx, action_rewards) in position_rewards.iter().enumerate() {
            println!("  Action {}: {:?}", all_actions[action_idx], action_rewards);
        }
    }
}

pub fn score(agent_position: i64, terminal_position: &Vec<i64>) -> f64 {
    let mut score: f64 = 0.0;
    if agent_position == terminal_position[0] {
        score = -1.0
    }
    if agent_position == terminal_position[1] {
        score = 1.0;
    }
    score
}


use std::collections::HashMap;

// Définir une structure pour encapsuler les deux types de politiques
pub enum Policy {
    Array(Vec<Action>),
    Map(HashMap<State, Action>),
}

// Fonction pour utiliser la politique dans le jeu
pub fn use_policy_in_game<E: Environment>(env: &mut E, policy: Policy) {
    println!("The Game start!");
    env.display();

    match policy {
        Policy::Array(actions) => {
            for (step_id, action) in actions.iter().enumerate() {
                if step_id >= env.state_id() as usize && !env.is_game_over() {
                    println!("State {} : action {}", step_id, action);
                    if step_id == env.state_id() as usize {
                        env.step(action.clone() as Action);
                        env.display();
                    }
                } else if env.is_game_over() {
                    println!("Game Over!");
                    println!("Score : {}", env.score());
                    break;
                }
            }
        }
        Policy::Map(action_map) => {
            while !env.is_game_over() {
                let state_id = env.state_id();
                if let Some(action) = action_map.get(&state_id) {
                    println!("State {} : action {}", state_id, action);
                    env.step(action.clone() as Action);
                    env.display();
                } else {
                    println!("No action defined for state {}. Game Over!", state_id);
                    break;
                }
            }
            println!("Game Over!");
            println!("Score : {}", env.score());
        }
    }
}

pub fn secret_env_lib() -> Library {
    unsafe {
        #[cfg(target_os = "linux")]
        let path = "src/libs/libsecret_envs.so";
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        let path = "src/libs/libsecret_envs_intel_macos.dylib";
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        let path = "src/libs/libsecret_envs.dylib";
        #[cfg(windows)]
        let path = "src/libs/secret_envs.dll";
        let lib = libloading::Library::new(path).expect("Failed to load library");
        lib
    }
}