use super::nn_struct::NeuralNetwork;

#[no_mangle]
pub(crate) extern "C" fn propagate(model: &mut NeuralNetwork, inputs: *const f32, inputs_len: i32, is_classification: bool) {
    // Convert arr to slice
    let inputs_slice = unsafe { std::slice::from_raw_parts(inputs, inputs_len as usize) };

    // Fill inputs (skip the bias term at index 0)
    for i in 0..model.neurons_per_layer[0] {
        model.activations[0][i + 1] = inputs_slice[i];
    }

    // Update neuron output, layer after layer
    for layer in 1..model.num_layers {
        for j in 1..=model.neurons_per_layer[layer] {
            let mut total = 0.0;
            for i in 0..=model.neurons_per_layer[layer - 1] {
                total += model.weights[layer][i][j] * model.activations[layer - 1][i];
            }

            if layer < model.num_layers - 1 { // Hidden layers
                if is_classification {
                    // Apply tanh activation to hidden layers
                    total = total.tanh();
                } else {
                    // Apply ReLU activation to hidden layers
                    total = total.max(0.0);
                }
            } else { // Output layer
                if !is_classification {
                    // Linear activation for output layer (no change)
                    // This is already the case, so no need to do anything special here
                } else {
                    // Apply tanh activation to output layer
                    total = total.tanh();
                }
            }

            model.activations[layer][j] = total;
        }
    }
}