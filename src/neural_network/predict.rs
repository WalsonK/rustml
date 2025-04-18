use crate::basic_function::tools::free_clone_neural_network_ptr;
use super::nn_struct::NeuralNetwork;
use super::propagate::propagate;


#[no_mangle]
pub(crate) extern "C" fn predict(model: &mut NeuralNetwork, inputs: *const f32, inputs_len: i32, is_classification: bool) -> *mut [f32] {
    let clone_model_box = Box::new(model.clone());
    let clone_ptr = Box::into_raw(clone_model_box);
    let clone_model = unsafe {&mut *clone_ptr};

    propagate(clone_model, inputs, inputs_len, is_classification);

    // Clone the output layer activations (excluding bias)
    let output_layer = &clone_model.activations[clone_model.num_layers - 1][1..];
    let output_vec = output_layer.to_vec();
    let output_box = output_vec.into_boxed_slice();

    // Return the pointer to the cloned slice
    let res = Box::into_raw(output_box);

    free_clone_neural_network_ptr(clone_ptr);
    res
}