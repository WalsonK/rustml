use rand::Rng;
use super::pmc_struct::PMC;
use super::propagate::propagate;

#[no_mangle]
extern "C" fn train_pmc_model(model: &mut PMC,
                              inputs_ptr: *const f32, input_length: i32, inputs_sub: i32,
                              output_ptr: *const f32, output_length: i32, output_sub: i32,
                              valid_ptr: *const f32, valid_length: i32, valid_sub: i32,
                              label_ptr: *const f32, label_length: i32, label_sub: i32,
                              learning_rate: f32, iteration: i32, is_classification: bool) {
    // Recompose Vec<Vec<f32>>
    let inputs = recompose_2d_vec(inputs_ptr, input_length, inputs_sub);
    let outputs = recompose_2d_vec(output_ptr, output_length, output_sub);
    let validations = recompose_2d_vec(valid_ptr, valid_length, valid_sub);
    let labels = recompose_2d_vec(label_ptr, label_length, label_sub);
    let vals = (validations, labels);

    let mut rng = rand::thread_rng();

    for _ in 0..iteration as usize {
        // pick random data in the dataset
        let rand = rng.gen_range(0..inputs.len());
        let rand_input = inputs[rand].clone();
        let rand_output = outputs[rand].clone();

        let rand_val  =  rng.gen_range(0..vals.0.len());
        let val_rand_inputs = vals.0[rand_val].clone();
        let val_rand_outputs = vals.1[rand_val].clone();

        let input_slice = rand_input.as_slice();

        // Set neurons inputs with the random dataset
        propagate(model, input_slice.as_ptr(), rand_input.len() as i32, is_classification);

        // Calc semi gradient last layer
        for j in 1..=model.neurons_per_layer[model.layers] {
            model.deltas[model.layers][j] = (model.neuron_data[model.layers][j] - rand_output[j - 1]);
            if is_classification {
                model.deltas[model.layers][j] *= (1.0 - model.neuron_data[model.layers][j].powf(2.0));
            }
        }

        // Calc other layer
        for layer in (1..=model.layers).rev() {
            for i in 1..= model.neurons_per_layer[layer] {
                let mut total: f32 = 0.0;
                for j in 1..=model.neurons_per_layer[layer] {
                    total += model.weights[layer][i][j] * model.deltas[layer][j];
                }
                model.deltas[layer - 1][i] = (1.0 - model.neuron_data[layer - 1][i].powf(2.0)) * total;
            }
        }

        // Update Weights
        for layer in 1..=model.layers {
            for i in 0..=model.neurons_per_layer[layer -1]{
                for j in 1..=model.neurons_per_layer[layer] {
                    model.weights[layer][i][j] -= learning_rate * model.neuron_data[layer - 1][i] * model.deltas[layer][j];
                }
            }
        }

    }
}

pub fn recompose_2d_vec(ptr: *const f32, array_length: i32, sub_array: i32) -> Vec<Vec<f32>>{
    let slice = unsafe { std::slice::from_raw_parts(ptr, array_length as usize) };
    let mut result = Vec::new();

    for chunk in slice.chunks(sub_array as usize) {
        result.push(chunk.to_vec());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recompose_array() {
        // Inputs Data
        let input_data: Vec<f32> = vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0
        ];
        let array_length = input_data.len() as i32;
        let sub_array = 4;
        let ptr = input_data.as_ptr();
        let result = recompose_2d_vec(ptr, array_length, sub_array);

        // Expected data
        let expected_result = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 10.0, 11.0, 12.0]
        ];
        // Check
        assert_eq!(result, expected_result, "Recompose Vec don't work");
    }
}