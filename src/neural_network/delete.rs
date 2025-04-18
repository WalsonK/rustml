use super::nn_struct::NeuralNetwork;

#[no_mangle]
extern "C" fn delete_pmc(model: &mut NeuralNetwork) {
    unsafe {
        let _ = Box::from_raw(model);
    }
    println!("Model deleted")
}