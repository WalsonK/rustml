use libloading::{Library, Symbol};
use std::os::raw::c_void;
use crate::environment::environment::{State, Action, Reward, Environment};

pub struct SecretEnv3Dp {
    lib: Library,
    env: *mut c_void,
    num_states: usize,
    num_actions: usize,
    num_rewards: usize,
}

impl SecretEnv3Dp {
    pub unsafe fn new() -> Box<Self> {
        let lib = tools::secret_env_lib();
        let env = Self::create_new_env(&lib);
        let num_states = Self::get_num_states(&lib);
        let num_actions = Self::get_num_actions(&lib);
        let num_rewards = Self::get_num_rewards(&lib);

        Box::new(Self {
            lib,
            env,
            num_states,
            num_actions,
            num_rewards,
        })
    }

    unsafe fn create_new_env(lib: &Library) -> *mut c_void {
        let secret_env_3_new: Symbol<unsafe extern fn() -> *mut c_void> =
            lib.get(b"secret_env_3_new").expect("Failed to load function `secret_env_3_new`");
        secret_env_3_new()
    }

    unsafe fn get_num_states(lib: &Library) -> usize {
        let secret_env_3_num_states: Symbol<unsafe extern fn() -> usize> =
            lib.get(b"secret_env_3_num_states").expect("Failed to load function `secret_env_3_num_states`");
        secret_env_3_num_states()
    }

    unsafe fn get_num_actions(lib: &Library) -> usize {
        let secret_env_3_num_actions: Symbol<unsafe extern fn() -> usize> =
            lib.get(b"secret_env_3_num_actions").expect("Failed to load function `secret_env_3_num_actions`");
        secret_env_3_num_actions()
    }

    pub fn get_num_rewards(lib: &Library) -> usize {
        unsafe {
            let secret_env_3_num_rewards: Symbol<unsafe extern fn() -> usize> =
                lib.get(b"secret_env_3_num_rewards").expect("Failed to load function `secret_env_3_num_rewards`");
            secret_env_3_num_rewards()
        }
    }
}

impl Environment for SecretEnv3Dp {
    fn reset(&mut self) -> State {
        unsafe {
            let secret_env_3_reset: Symbol<unsafe extern fn(*mut c_void)> =
                self.lib.get(b"secret_env_3_reset").expect("Failed to load function `secret_env_3_reset`");
            secret_env_3_reset(self.env);

            let secret_env_3_state_id: Symbol<unsafe extern fn(*const c_void) -> usize> =
                self.lib.get(b"secret_env_3_state_id").expect("Failed to load function `secret_env_3_state_id`");
            secret_env_3_state_id(self.env)
        }
    }

    fn step(&mut self, action: Action) -> (State, Reward, bool) {
        unsafe {
            let secret_env_3_step: Symbol<unsafe extern fn(*mut c_void, usize)> =
                self.lib.get(b"secret_env_3_step").expect("Failed to load function `secret_env_3_step`");
            secret_env_3_step(self.env, action);

            let secret_env_3_state_id: Symbol<unsafe extern fn(*const c_void) -> usize> =
                self.lib.get(b"secret_env_3_state_id").expect("Failed to load function `secret_env_3_state_id`");
            let state_id = secret_env_3_state_id(self.env);

            let secret_env_3_score: Symbol<unsafe extern fn(*const c_void) -> f32> =
                self.lib.get(b"secret_env_3_score").expect("Failed to load function `secret_env_3_score`");
            let reward = secret_env_3_score(self.env);

            let is_game_over = self.is_game_over();
            (state_id, reward, is_game_over)
        }
    }

    fn available_actions(&self) -> Vec<Action> {
        unsafe {
            let secret_env_3_available_actions: Symbol<unsafe extern fn(*const c_void) -> *const usize> =
                self.lib.get(b"secret_env_3_available_actions").expect("Failed to load function `secret_env_3_available_actions`");
            let actions_ptr = secret_env_3_available_actions(self.env);

            let secret_env_3_available_actions_len: Symbol<unsafe extern fn(*const c_void) -> usize> =
                self.lib.get(b"secret_env_3_available_actions_len").expect("Failed to load function `secret_env_3_available_actions_len`");
            let len = secret_env_3_available_actions_len(self.env);
            let actions_slice = std::slice::from_raw_parts(actions_ptr, len);
            actions_slice.iter().map(|&x| x as Action).collect()
        }
    }

    fn all_states(&self) -> Vec<State> {
        (0..self.num_states).collect()
    }

    fn all_action(&self) -> Vec<Action> {
        (0..self.num_actions).collect()
    }

    fn set_state(&mut self, state: State) {
        unsafe {
            let secret_env_3_set_state: Symbol<unsafe extern fn(*mut c_void, usize)> =
                self.lib.get(b"secret_env_3_set_state").expect("Failed to load function `secret_env_3_set_state`");
            secret_env_3_set_state(self.env, state);
        }
    }

    fn is_forbidden(&self, state_or_action: usize) -> bool {
        unsafe {
            let secret_env_3_is_forbidden: Symbol<unsafe extern fn(*const c_void, usize) -> bool> =
                self.lib.get(b"secret_env_3_is_forbidden").expect("Failed to load function `secret_env_3_is_forbidden`");
            secret_env_3_is_forbidden(self.env, state_or_action)
        }
    }

    fn display(&self) {
        unsafe {
            let secret_env_3_display: Symbol<unsafe extern fn(*const c_void)> =
                self.lib.get(b"secret_env_3_display").expect("Failed to load function `secret_env_3_display`");
            secret_env_3_display(self.env);
        }
    }

    fn state_id(&self) -> State {
        unsafe {
            let secret_env_3_state_id: Symbol<unsafe extern fn(*const c_void) -> usize> =
                self.lib.get(b"secret_env_3_state_id").expect("Failed to load function `secret_env_3_state_id`");
            secret_env_3_state_id(self.env)
        }
    }

    fn score(&self) -> Reward {
        unsafe {
            let secret_env_3_score: Symbol<unsafe extern fn(*const c_void) -> f32> =
                self.lib.get(b"secret_env_3_score").expect("Failed to load function `secret_env_3_score`");
            secret_env_3_score(self.env)
        }
    }

    fn is_game_over(&self) -> bool {
        unsafe {
            let secret_env_3_is_game_over: Symbol<unsafe extern fn(*const c_void) -> bool> =
                self.lib.get(b"secret_env_3_is_game_over").expect("Failed to load function `secret_env_3_is_game_over`");
            secret_env_3_is_game_over(self.env)
        }
    }
}

mod tools {
    use libloading::Library;

    pub unsafe fn secret_env_lib() -> Library {
        let lib_path = r#"C:\Users\farin\CLionProjects\rustml2\src\libs\secret_envs.dll"#;
        Library::new(lib_path).expect("Failed to load library")
    }
}
