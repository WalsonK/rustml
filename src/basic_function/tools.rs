use crate::neural_network::nn_struct::NeuralNetwork;

pub fn recompose_2d_vec(ptr: *const f32, array_length: i32, sub_array: i32) -> Vec<Vec<f32>>{
    let slice = unsafe { std::slice::from_raw_parts(ptr, array_length as usize) };
    let mut result = Vec::new();

    for chunk in slice.chunks(sub_array as usize) {
        result.push(chunk.to_vec());
    }

    result
}

pub fn recompose_vec<T>(ptr: *const T, array_length: i32) -> Vec<T> where T: Clone, {
    let slice = unsafe { std::slice::from_raw_parts(ptr, array_length as usize) };
    slice.to_vec()
}

pub fn argmax(vector: &Vec<f32>) -> i32 {
    assert!(!vector.is_empty(), "The vector should not be empty.");

    let mut max_index = 0;
    let mut max_value = f32::NEG_INFINITY;

    for (index, &value) in vector.iter().enumerate() {
        if value.is_nan() {
            continue;
        }
        if value > max_value {
            max_value = value;
            max_index = index;
        }
    }

    max_index as i32
}

pub fn clone_neural_network_ptr(original_model : *mut NeuralNetwork) -> *mut NeuralNetwork {
    let clone = unsafe {
        let original = &*original_model;      // Convert raw pointer to reference
        Box::new(original.clone())                           // Clone the NeuralNetwork instance
    };
    Box::into_raw(clone)                                     // Convert the Box to a raw pointer
}

// Convert raw pointer back to Box and drop it to free memory
pub fn free_clone_neural_network_ptr(ptr : *mut NeuralNetwork) {
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}


pub(crate) fn free_vec(ptr: *mut [f32]) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recompose_2d_array() {
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

    #[test]
    fn test_recompose_array() {
        // F32
        let input_data_f32: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
        let ptr_f32 = input_data_f32.as_ptr();
        let array_length_f32 = input_data_f32.len() as i32;
        let result_f32 = recompose_vec(ptr_f32, array_length_f32);
        let expected_result_f32 = input_data_f32.clone();

        // I32
        let input_data_i32: Vec<i32> = vec![1, 2, 3, 4];
        let ptr_i32 = input_data_i32.as_ptr();
        let array_length_i32 = input_data_i32.len() as i32;
        let result_i32 = recompose_vec(ptr_i32, array_length_i32);
        let expected_result_i32 = input_data_i32.clone();

        assert_eq!(result_f32, expected_result_f32, "Recompose Vec don't work");
        assert_eq!(result_i32, expected_result_i32, "Recompose Vec don't work");
    }

    #[test]
    fn test_argmax() {
        let input_data: Vec<f32> = vec![1.0, 4.0, 3.0, 2.0];
        let index_max = argmax(&input_data);
        assert_eq!(index_max, 1);
    }
}