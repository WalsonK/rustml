use super::dql_struct::DeepQLearning;

#[no_mangle]
pub (crate) extern "C" fn delete_dql(model: &mut DeepQLearning) {
    unsafe {
        let _ = Box::from_raw(model);
    }
    println!("Model deleted")
}