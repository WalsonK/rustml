use super::dql_struct::DeepQLearning;
use crate::basic_function::tools::{recompose_vec, argmax, free_vec};
use crate::neural_network::{ predict::predict, train::one_step_train_pmc2 };

#[no_mangle]
pub (crate) extern "C" fn update_epsilon(model: &mut DeepQLearning) {
    model.epsilon = model.epsilon_min.max(model.epsilon * model.epsilon_decay);
}

#[no_mangle]
pub (crate) extern "C" fn learn_dql(model: &mut DeepQLearning, state: *const f32, state_len: i32, action: i32, reward: i32, next_state: *const f32, next_state_len: i32, output_len: i32, done: bool) {
    // Recompose vec
    let next_state_vec = recompose_vec::<f32>(next_state, next_state_len);

    // Vec to useful vars
    let next_state_ptr = next_state_vec.as_ptr();

    let mut target = reward as f32;

    let nn_ref = &mut *model.neural_network;

    // Calc target value based on next q values
    if !done {
        let next_q_values_ptr = predict(
            nn_ref,
            next_state_ptr,
            next_state_len,
            false
        );
        let next_q_values_vec = unsafe { std::slice::from_raw_parts(next_q_values_ptr as *mut f32, output_len as usize).to_vec() };
        let argmax_next_q_value = argmax(&next_q_values_vec);

        target += model.gamma * next_q_values_vec[argmax_next_q_value as usize];

        free_vec(next_q_values_ptr);
    }

    // Calc current q values
    let current_q_value_ptr = predict(
        nn_ref,
        state,
        state_len,
        false
    );
    let mut current_q_value_vec = unsafe { std::slice::from_raw_parts(current_q_value_ptr as *mut f32, output_len as usize).to_vec() };

    current_q_value_vec[action as usize] = target;

    // Train with current q Values, target in q_values[0]
    one_step_train_pmc2(
        nn_ref,
        state,
        state_len,
        current_q_value_vec.as_mut_ptr(),
        output_len,
        model.learning_rate,
        false
    );

    free_vec(current_q_value_ptr);
}