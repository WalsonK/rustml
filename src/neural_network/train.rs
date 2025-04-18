use rand::Rng;
use super::nn_struct::NeuralNetwork;
use super::propagate::propagate;
use crate::basic_function::tools::{recompose_2d_vec, recompose_vec};

#[no_mangle]
pub (crate) extern "C" fn train_pmc_model(model: &mut NeuralNetwork,
                              inputs_ptr: *const f32, input_length: i32, inputs_sub: i32,
                              output_ptr: *const f32, output_length: i32, output_sub: i32,
                              valid_ptr: *const f32, valid_length: i32, valid_sub: i32,
                              label_ptr: *const f32, label_length: i32, label_sub: i32,
                              learning_rate: f32, iteration: i32, is_classification: bool) {
    // Recompose Vec<Vec<f32>>
    let inputs =  recompose_2d_vec(inputs_ptr, input_length, inputs_sub);
    let outputs = recompose_2d_vec(output_ptr, output_length, output_sub);
    let validations = recompose_2d_vec(valid_ptr, valid_length, valid_sub);
    let labels = recompose_2d_vec(label_ptr, label_length, label_sub);
    let vals = (validations, labels);

    let mut rng = rand::rng();

    for _ in 0..iteration as usize {
        // pick random data in the dataset
        let rand = rng.random_range(0..inputs.len());
        let rand_input = inputs[rand].clone();
        let rand_output = outputs[rand].clone();

        let rand_val  =  rng.random_range(0..vals.0.len());
        let _val_rand_inputs = vals.0[rand_val].clone();
        let _val_rand_outputs = vals.1[rand_val].clone();

        let input_slice = rand_input.as_slice();

        // Set neurons inputs with the random dataset
        propagate(model, input_slice.as_ptr(), rand_input.len() as i32, is_classification);

        // Calc semi gradient last layer
        for j in 1..=model.neurons_per_layer[model.num_layers] {
            model.deltas[model.num_layers][j] = model.activations[model.num_layers][j] - rand_output[j - 1];
            if is_classification {
                model.deltas[model.num_layers][j] *= 1.0 - model.activations[model.num_layers][j].powf(2.0);
            }
        }

        // Calc other layer
        for layer in (1..=model.num_layers).rev() {
            for i in 1..= model.neurons_per_layer[layer] {
                let mut total: f32 = 0.0;
                for j in 1..=model.neurons_per_layer[layer] {
                    total += model.weights[layer][i][j] * model.deltas[layer][j];
                }
                model.deltas[layer - 1][i] = (1.0 - model.activations[layer - 1][i].powf(2.0)) * total;
            }
        }

        // Update Weights
        for layer in 1..=model.num_layers {
            for i in 0..=model.neurons_per_layer[layer -1]{
                for j in 1..=model.neurons_per_layer[layer] {
                    model.weights[layer][i][j] -= learning_rate * model.activations[layer - 1][i] * model.deltas[layer][j];
                }
            }
        }

    }
}

#[no_mangle]
pub (crate) extern "C" fn one_step_train_pmc(
    model: &mut NeuralNetwork,
    inputs_ptr: *const f32, input_length: i32,
    output_ptr: *const f32, output_length: i32,
    learning_rate: f32, is_classification: bool)
{
    // Recompose Vec<f32>
    let outputs = recompose_vec(output_ptr, output_length);

    // Set neurons inputs
    propagate(model, inputs_ptr, input_length, is_classification);

    // Calc semi gradient last layer
    for j in 1..=model.neurons_per_layer[model.num_layers] {
        model.deltas[model.num_layers][j] = model.activations[model.num_layers][j] - outputs[j - 1];
        if is_classification {
            model.deltas[model.num_layers][j] *= 1.0 - model.activations[model.num_layers][j].powf(2.0);
        }
    }

    // Calc other layer
    for layer in (1..=model.num_layers).rev() {
        for i in 1..= model.neurons_per_layer[layer] {
            let mut total: f32 = 0.0;
            for j in 1..=model.neurons_per_layer[layer] {
                total += model.weights[layer][i][j] * model.deltas[layer][j];
            }
            model.deltas[layer - 1][i] = (1.0 - model.activations[layer - 1][i].powf(2.0)) * total;
        }
    }

    // Update Weights
    for layer in 1..=model.num_layers {
        for i in 0..=model.neurons_per_layer[layer -1]{
            for j in 1..=model.neurons_per_layer[layer] {
                model.weights[layer][i][j] -= learning_rate * model.activations[layer - 1][i] * model.deltas[layer][j];
            }
        }
    }
}

pub(crate) extern "C" fn one_step_train_pmc2(
    model: &mut NeuralNetwork,
    inputs_ptr: *const f32,
    input_length: i32,
    output_ptr: *const f32,
    output_length: i32,
    learning_rate: f32,
    is_classification: bool,
) {
    // Recompose Vec<f32> for outputs
    let outputs = unsafe { std::slice::from_raw_parts(output_ptr, output_length as usize).to_vec() };
    let num_layers_with_no_input = model.num_layers - 1;

    // Set neurons inputs and propagate
    propagate(model, inputs_ptr, input_length, is_classification);

    // Calculate semi-gradient for the last layer (output layer)
    for j in 1..model.neurons_per_layer[num_layers_with_no_input] {
        let activation = model.activations[num_layers_with_no_input][j];
        let error = activation - outputs[j - 1]; // Zero-based index for outputs
        if is_classification {
            // Derivative of tanh: 1 - tanh^2(x)
            model.deltas[num_layers_with_no_input][j] = error * (1.0 - activation.powf(2.0));
        } else {
            // For regression, use the error directly
            model.deltas[num_layers_with_no_input][j] = error;
        }
    }

    // Calculate deltas for other layers
    for layer in (1..num_layers_with_no_input).rev() {
        for i in 1..=model.neurons_per_layer[layer] {
            let mut total: f32 = 0.0;
            for j in 1..=model.neurons_per_layer[layer + 1] {
                total += model.weights[layer + 1][i][j] * model.deltas[layer + 1][j];
            }
            let activation = model.activations[layer][i];
            if is_classification {
                // Derivative of tanh: 1 - tanh^2(x)
                model.deltas[layer][i] = total * (1.0 - activation.powf(2.0));
            } else {
                // For ReLU, derivative is 1 if activation > 0, otherwise 0
                model.deltas[layer][i] = total * if activation > 0.0 { 1.0 } else { 0.0 };
            }
        }
    }

    // Update weights
    for layer in 1..num_layers_with_no_input {
        for i in 0..=model.neurons_per_layer[layer - 1] {
            for j in 1..=model.neurons_per_layer[layer] {
                model.weights[layer][i][j] -= learning_rate * model.activations[layer - 1][i] * model.deltas[layer][j];
            }
        }
    }
}