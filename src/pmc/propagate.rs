use super::pmc_struct::*;

#[no_mangle]
pub(crate) extern "C" fn propagate(model: &mut PMC, inputs: *const f32, inputs_len: i32, is_classification: bool) {
    // Convert arr to slice
    let inputs_slice = unsafe { std::slice::from_raw_parts(inputs, inputs_len as usize) };

    // Fill inputs
    for i in 0..model.neurons_per_layer[0] {
        model.neuron_data[0][i + 1] = inputs_slice[i];
    }

    // Update neuron output, layer after layer
    for layer in 1..=model.layers {
        for j in 1..=model.neurons_per_layer[layer] {
            let mut total = 0.0;
            for i in 0..=model.neurons_per_layer[layer - 1] {
                total += model.weights[layer][i][j] * model.neuron_data[layer - 1][i];
            }
            if layer < model.layers || is_classification {
                total = total.tanh();
            }
            model.neuron_data[layer][j] = total;
        }
    }
}