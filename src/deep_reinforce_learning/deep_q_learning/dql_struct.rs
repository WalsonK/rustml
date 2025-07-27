use bincode::{Decode, Encode};
use crate::neural_network::nn_struct::NeuralNetwork;

#[repr(C)]
#[derive(Encode, Decode, PartialEq, Debug)]
pub struct DeepQLearning {
    pub self_ptr: usize,
    pub state_size: i32,
    pub action_size: i32,
    pub learning_rate: f32,
    pub gamma: f32,
    pub epsilon: f32,
    pub epsilon_min: f32,
    pub epsilon_decay: f32,
    pub neural_network: Box<NeuralNetwork>,
}