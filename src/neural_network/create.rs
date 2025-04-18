use super::nn_struct::NeuralNetwork;
use rand::Rng;

#[no_mangle]
pub(crate) extern "C" fn init(arr: *const i32, len: i32) -> Box<NeuralNetwork> {
    // Convert the raw pointer to a slice
    let arr_slice = unsafe { std::slice::from_raw_parts(arr, len as usize) };

    // Initialize the neural_network model
    let mut model = Box::new(NeuralNetwork {
        num_layers: len as usize,
        neurons_per_layer: arr_slice.iter().map(|&x| x as usize).collect(),
        weights: Vec::new(),
        activations: Vec::new(),
        deltas: Vec::new()
    });

    // Create a random number generator
    let mut rng = rand::rng();

    for layer in 0..model.num_layers {
        if layer == 0 {
            // No weights for the input layer, just initialize an empty vector
            model.weights.push(Vec::new());
        } else {
            // Initialize weights with bias terms
            let previous_layer_neurons = model.neurons_per_layer[layer - 1];
            let current_layer_neurons = model.neurons_per_layer[layer];
            let layer_weights: Vec<Vec<f32>> = (0..=previous_layer_neurons)
                .map(|_| {
                    (0..=current_layer_neurons)
                        .map(|j| if j == 0 { 0.0f32 } else { rng.random_range(-1.0..=1.0) })
                        .collect()
                })
                .collect();
            model.weights.push(layer_weights);
        }

        // Initialize activations and deltas for the current layer
        let current_layer_neurons = model.neurons_per_layer[layer];
        let layer_activations: Vec<f32> = (0..=current_layer_neurons)
            .map(|i| if i == 0 { 1.0 } else { 0.0 })
            .collect();
        let layer_deltas: Vec<f32> = vec![0.0; current_layer_neurons + 1];
        model.activations.push(layer_activations);
        model.deltas.push(layer_deltas);
    }

    model
}

#[cfg(test)]
mod init_tests_simple {
    use super::*;

    // TEST DATA
    fn setup_model() -> Box<NeuralNetwork> {
        // Init Data
        let slice: &[i32] = &[3, 2, 1];
        let ptr: *const i32 = slice.as_ptr();
        let len: i32 = slice.len() as i32;

        let model = init(ptr, len);
        model
    }

    #[test]
    fn init_pmc() {
        let model = setup_model();
        assert_eq!(model.neurons_per_layer, vec![3, 2, 1]);
    }

    #[test]
    fn init_weight() {
        let model = setup_model();
        // layers
        assert_eq!(model.num_layers, 2);
        // Weights[0]
        assert!(model.weights[0].is_empty());
        // Weights[1]
        assert_eq!(model.weights[1].len(), 4);
        for neuron_weights in &model.weights[1] {
            assert_eq!(neuron_weights.len(), 3);
            assert_eq!(neuron_weights[0], 0.0);
            for &weight in &neuron_weights[1..] {
                assert!(weight >= -1.0 && weight <= 1.0);
            }
        }
        // Weights[2]
        assert_eq!(model.weights[2].len(), 3);
        for neuron_weights in &model.weights[2] {
            assert_eq!(neuron_weights.len(), 2);
            assert_eq!(neuron_weights[0], 0.0);
            for &weight in &neuron_weights[1..] {
                assert!(weight >= -1.0 && weight <= 1.0);
            }
        }
    }

    #[test]
    fn init_neuron_data() {
        let model = setup_model();
        // Len
        assert_eq!(model.activations.len(), 3);
        // First Layer
        assert_eq!(model.activations[0], vec![1.0, 0.0, 0.0, 0.0]);
        // Second Layer
        assert_eq!(model.activations[1], vec![1.0, 0.0, 0.0]);
        // Third Layer
        assert_eq!(model.activations[2], vec![1.0, 0.0]);
    }

    #[test]
    fn init_delta() {
        let model = setup_model();
        // Len
        assert_eq!(model.deltas.len(), 3);
        // First Layer
        assert_eq!(model.deltas[0], vec![0.0, 0.0, 0.0, 0.0]);
        // Second Layer
        assert_eq!(model.deltas[1], vec![0.0, 0.0, 0.0]);
        // Third Layer
        assert_eq!(model.deltas[2], vec![0.0, 0.0]);
    }
}