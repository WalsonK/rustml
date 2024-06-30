use libloading::Library;
use crate::environment::{tools, secret_wrapper};

pub struct SecretEnv0Dp {
    pub num_states: usize,
    pub num_actions: usize,
    pub num_rewards: usize,
    lib: Library
}

impl SecretEnv0Dp {
    pub fn new() -> Box<SecretEnv0Dp> {
        let lib = tools::secret_env_lib();
        let mut env = Box::new(SecretEnv0Dp {
            num_states: 0,
            num_actions: 0,
            num_rewards: 0,
            lib
        });

        env.num_states = env.get_num_states();
        env.num_actions = env.get_num_actions();
        env.num_rewards = env.get_num_rewards();

        env
    }

    fn get_num_states(&self) -> usize {
        unsafe {
            let secret_env_0_num_states: libloading::Symbol<unsafe extern fn() -> usize> =
                self.lib.get(b"secret_env_0_num_states")
                    .expect("Failed to load function `secret_env_0_num_states`");

            secret_env_0_num_states()
        }
    }

    fn get_num_actions(&self) -> usize {
        unsafe {
            let secret_env_0_num_actions: libloading::Symbol<unsafe extern fn() -> usize> =
                self.lib.get(b"secret_env_0_num_actions")
                    .expect("Failed to load function `secret_env_0_num_actions`");

            secret_env_0_num_actions()
        }
    }

    fn get_num_rewards(&self) -> usize {
        unsafe {
            let secret_env_0_num_rewards: libloading::Symbol<unsafe extern fn() -> usize> =
                self.lib.get(b"secret_env_0_num_rewards")
                    .expect("Failed to load function `secret_env_0_num_rewards`");

            secret_env_0_num_rewards()
        }
    }

    pub fn load_rewards(&self) -> Vec<f32> {
        let mut rewards: Vec<f32> = Vec::with_capacity(self.num_rewards);
        unsafe {
            let secret_env_0_reward: libloading::Symbol<unsafe extern fn(usize) -> f32> =
                self.lib.get(b"secret_env_0_reward")
                    .expect("Failed to load function `secret_env_0_reward`");

            for i in 0..self.num_rewards {
                rewards.push(secret_env_0_reward(i));
            }
        }
        rewards
    }
}