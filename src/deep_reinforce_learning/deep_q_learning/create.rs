use super::dql_struct::DeepQLearning;
use crate::neural_network::create::init;

#[no_mangle]
pub(crate) extern "C" fn init_dql(
    state_dim: i32,
    action_dim: i32,
    learning_rate: f32,
    gamma: f32,
    epsilon: f32,
    epsilon_min: f32,
    epsilon_decay: f32
) -> Box<DeepQLearning> {
    // Init neural network
    let arr = [state_dim, 64, 64, action_dim];
    let ptr = arr.as_ptr();
    let len = arr.len() as i32;

    let model = Box::new(DeepQLearning {
        self_ptr: 0,
        state_size: state_dim,
        action_size: action_dim,
        learning_rate,
        gamma,
        epsilon,
        epsilon_min,
        epsilon_decay,
        neural_network: init(ptr, len)
    });

    model
}