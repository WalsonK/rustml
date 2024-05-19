use super::pmc_struct::*;
use rand::Rng;

#[no_mangle]
extern "C" fn init(arr: *const i32, len: i32) -> Box<PMC> {
    // convert arr to slice
    let arr_slice = unsafe { std::slice::from_raw_parts(arr, len as usize) };

    // PMC model Initialisation
    let mut model = Box::new(PMC {
        layers: (len - 1) as usize,
        neurons_per_layer: arr_slice.iter().map(|&x| x as usize).collect(),
        weights : Vec::new(),
        neuron_data: Vec::new(),
        deltas: Vec::new()
    });

    // Weights Initialisation
    let mut rng = rand::thread_rng();
    for layer in 0..=model.layers {
        let mut layer_weights = Vec::new();

        if layer == 0 {
            model.weights.push(layer_weights);
        } else {
            for _ in 0..=model.neurons_per_layer[layer - 1] {
                let mut neuron_weights = Vec::new();

                for j in 0..=model.neurons_per_layer[layer] {
                    let weight = if j == 0 { 0.0f32 } else { rng.gen_range(-1.0..=1.0) };
                    neuron_weights.push(weight);
                }
                layer_weights.push(neuron_weights);
            }
            model.weights.push(layer_weights);
        }
    }

    // Neuron Data Initialisation
    for layer in 0..=model.layers {
        let mut layer_data = Vec::new();
        for i in 0..=model.neurons_per_layer[layer] {
            let value = if i == 0 { 1.0 } else { 0.0 };
            layer_data.push(value);
        }
        model.neuron_data.push(layer_data);
    }

    // Deltas Initialisation
    for layer in 0..=model.layers {
        let mut layer_deltas = Vec::new();
        for _ in 0..=model.neurons_per_layer[layer]{
            let delta: f32 = 0.0;
            layer_deltas.push(delta);
        }
        model.deltas.push(layer_deltas);
    }


    model

}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST DATA
    fn setup_model() -> Box<PMC> {
        // Init Data
        let slice: &[i32] = &[3, 2, 1];
        let ptr: *const i32 = slice.as_ptr();
        let len: i32 = slice.len() as i32;

        let model = init(ptr, len);
        model
    }

    #[test]
    fn init_pmc() {
        let model = setup_model();
        assert_eq!(model.neurons_per_layer, vec![3, 2, 1]);
    }

    #[test]
    fn init_weight() {
        let model = setup_model();
        // layers
        assert_eq!(model.layers, 2);
        // Weights[0]
        assert!(model.weights[0].is_empty());
        // Weights[1]
        assert_eq!(model.weights[1].len(), 4);
        for neuron_weights in &model.weights[1] {
            assert_eq!(neuron_weights.len(), 3);
            assert_eq!(neuron_weights[0], 0.0);
            for &weight in &neuron_weights[1..] {
                assert!(weight >= -1.0 && weight <= 1.0);
            }
        }
        // Weights[2]
        assert_eq!(model.weights[2].len(), 3);
        for neuron_weights in &model.weights[2] {
            assert_eq!(neuron_weights.len(), 2);
            assert_eq!(neuron_weights[0], 0.0);
            for &weight in &neuron_weights[1..] {
                assert!(weight >= -1.0 && weight <= 1.0);
            }
        }
    }

    #[test]
    fn init_neuron_data() {
        let model = setup_model();
        // Len
        assert_eq!(model.neuron_data.len(), 3);
        // First Layer
        assert_eq!(model.neuron_data[0], vec![1.0, 0.0, 0.0, 0.0]);
        // Second Layer
        assert_eq!(model.neuron_data[1], vec![1.0, 0.0, 0.0]);
        // Third Layer
        assert_eq!(model.neuron_data[2], vec![1.0, 0.0]);
    }

    #[test]
    fn init_delta() {
        let model = setup_model();
        // Len
        assert_eq!(model.deltas.len(), 3);
        // First Layer
        assert_eq!(model.deltas[0], vec![0.0, 0.0, 0.0, 0.0]);
        // Second Layer
        assert_eq!(model.deltas[1], vec![0.0, 0.0, 0.0]);
        // Third Layer
        assert_eq!(model.deltas[2], vec![0.0, 0.0]);
    }
}