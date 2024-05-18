use super::pmc_struct::*;
use rand::Rng;

#[no_mangle]
extern "C" fn init(arr: *const i32, len: i32) -> Box<PMC> {
    // convert arr to slice
    let arr_slice = unsafe { std::slice::from_raw_parts(arr, len as usize) };

    // init PMC
    let model = Box::new(PMC {
        layers: (len - 1) as usize,
        neurons_per_layer: arr_slice.iter().map(|&x| x as usize).collect(),
        weights : Vec::new(),
        neuron_data: Vec::new(),
        deltas: Vec::new()
    });

    // init Weights
    let mut rng = rand::thread_rng();
    for layer in 0..=model.layers {
        
    }

    model

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_pmc() {
        // Init Data
        let slice: &[i32] = &[10, 64, 32, 1];
        let ptr: *const i32 = slice.as_ptr();
        let len: i32 = slice.len() as i32;

        let model = init(ptr, len);

        assert_eq!(model.neurons_per_layer, vec![10, 64, 32, 1]);
    }
}