# [repr(C)]
pub struct PMC {
    pub layers: usize,
    pub neurons_per_layer: Vec<usize>,
    pub weights: Vec<Vec<Vec<f32>>>,
    pub neuron_data: Vec<Vec<f32>>,
    pub deltas: Vec<Vec<f32>>
}