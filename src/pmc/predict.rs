use super::pmc_struct::*;
use super::propagate::propagate;


#[no_mangle]
extern "C" fn predict(model: &mut PMC, inputs: *const f32, inputs_len: i32, is_classification: bool) -> *const f32{
    propagate(model, inputs, inputs_len, is_classification);
    let outputs = model.neuron_data[model.layers][1..].as_ptr();
    outputs
}