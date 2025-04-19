use super::dql_struct::DeepQLearning;
use crate::neural_network::predict::predict;
use crate::basic_function::tools::{argmax, free_vec, recompose_vec};
use rand::{rng, Rng};

#[no_mangle]
pub (crate) extern "C" fn choose_action(
    model: &mut DeepQLearning,
    inputs: *const f32,
    inputs_len: i32,
    output_len: i32,
    available_actions: *const i32,
    available_actions_len: i32,
) -> i32 {
    // Rebuild vec
    let available_actions_vec = recompose_vec::<i32>(available_actions, available_actions_len);

    let mut rng = rng();

    if rng.random::<f32>() <= model.epsilon {
        let index = rng.random_range(0..available_actions_vec.len());
        let action = available_actions_vec[index];
        action
    } else {
        unsafe {
            let mut_ref = &mut *model.neural_network;
            let q_value_ptr = predict(
                mut_ref,
                inputs,
                inputs_len,
                false
            );

            let q_value_vec = std::slice::from_raw_parts(q_value_ptr as *mut f32, output_len as usize).to_vec();

            let mut valid_q_values = vec![f32::NEG_INFINITY; output_len as usize];

            for action_index in 0..available_actions_vec.len() {
                valid_q_values[action_index] = q_value_vec[action_index]
            }

            let res = argmax(&valid_q_values);

            free_vec(q_value_ptr);
            
            res
        }
    }
}
