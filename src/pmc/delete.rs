use super::pmc_struct::*;

#[no_mangle]
extern "C" fn delete_pmc(model: &mut PMC) {
    unsafe {
        let _ = Box::from_raw(model);
    }
    println!("Model deleted")
}