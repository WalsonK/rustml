use bincode::{Decode, Encode};

#[repr(C)]
#[derive(Encode, Decode, PartialEq, Debug)]
pub struct NeuralNetwork {
    pub num_layers: usize,
    pub neurons_per_layer: Vec<usize>,
    pub weights: Vec<Vec<Vec<f32>>>,
    pub activations: Vec<Vec<f32>>,
    pub deltas: Vec<Vec<f32>>
}
impl Clone for NeuralNetwork {
    fn clone(&self) -> Self {
        NeuralNetwork {
            num_layers: self.num_layers,
            neurons_per_layer: self.neurons_per_layer.clone(),
            weights: self.weights.clone(),
            activations: self.activations.clone(),
            deltas: self.deltas.clone(),
        }
    }
}