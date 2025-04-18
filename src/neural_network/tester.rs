#[cfg(test)]
mod tests {
    use crate::neural_network::create::init;
    use crate::neural_network::nn_struct::NeuralNetwork;

    // TEST DATA
    fn setup_model() -> Box<NeuralNetwork> {
        // Init Data
        let slice = [10, 64, 64, 10];
        let ptr: *const i32 = slice.as_ptr();
        let len: i32 = slice.len() as i32;

        let model = init(ptr, len);
        model
    }

    #[test]
    fn init_pmc() {
        let model = setup_model();
        assert_eq!(model.neurons_per_layer, vec![10, 64, 64, 10]);
    }

    #[test]
    fn init_weight() {
        let model = setup_model();
        // layers
        assert_eq!(model.num_layers, 4);
        // Weights[0]
        assert!(model.weights[0].is_empty());
        // Weights[1]
        assert_eq!(model.weights[1].len(), 11);
        for neuron_weights in &model.weights[1] {
            assert_eq!(neuron_weights.len(), 65);
            assert_eq!(neuron_weights[0], 0.0);
            for &weight in &neuron_weights[1..] {
                assert!(weight >= -1.0 && weight <= 1.0);
            }
        }
        // Weights[2]
        assert_eq!(model.weights[2].len(), 65);
        for neuron_weights in &model.weights[2] {
            assert_eq!(neuron_weights.len(), 65);
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
        assert_eq!(model.activations.len(), 4);
        // First Layer
        assert_eq!(model.activations[0], vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        // Second Layer
        assert_eq!(model.activations[1], vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                              0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                              0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                              0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                              0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                              0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                              0.0, 0.0, 0.0, 0.0]);
        // Third Layer
        assert_eq!(model.activations[3], vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn init_delta() {
        let model = setup_model();
        // Len
        assert_eq!(model.deltas.len(), 4);
        // First Layer
        assert_eq!(model.deltas[0], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        // Second Layer
        assert_eq!(model.deltas[1], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                         0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                         0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                         0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                         0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                         0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                         0.0, 0.0, 0.0, 0.0]);
        // Third Layer
        assert_eq!(model.deltas[3], vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    /*
    #[test]
    fn test_clone_nn() {
        let model = setup_model();
        let original_nn_ptr = model.neural_network;
        let mut ptr2 = *original_nn_ptr.clone();

        // Clone the neural_network instance
        let clone_nn_ptr = clone_neural_network_ptr(*ptr2);
        let reference = unsafe { &mut *clone_nn_ptr };

        let state = [1.0f32, 2.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32, 7.0f32, 8.0f32, 9.0f32, 10.0f32];

        let _current_q_value_ptr = predict(
            reference,
            state.as_ptr(),
            state.len() as i32,
            false
        );

        // Print original and cloned networks after modification
        unsafe {
            println!("Original Network After Modification: {:?}", *original_nn_ptr);
            println!("Cloned Network After Modification: {:?}", *clone_nn_ptr);
        }

        // Assertion to ensure original network remains unchanged
        unsafe {
            assert_ne!(*original_nn_ptr, *clone_nn_ptr);
        }

        // Convert raw pointer back to Box and drop it to free memory
        free_clone_neural_network_ptr(clone_nn_ptr);
    }*/
}
