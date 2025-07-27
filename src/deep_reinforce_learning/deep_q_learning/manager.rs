use super::dql_struct::DeepQLearning;
use bincode::{config};

#[repr(C)]
pub struct BindModel {
    pub self_ptr: *mut u8,
    pub size: i32,
}
impl BindModel {
    fn new(vec: Vec<u8>) -> Self {
        let ptr = vec.as_ptr() as *mut u8;
        let len = vec.len();

        BindModel { self_ptr: ptr, size: len as i32 }
    }
}

#[no_mangle]
pub (crate) extern "C" fn check_size(model: &mut DeepQLearning) -> i32 {
    let size = std::mem::size_of_val::<DeepQLearning>(model);
    size as i32
}

#[no_mangle]
pub (crate) extern "C" fn save_model(model: &mut DeepQLearning) -> Box<BindModel> {
    // Serialize model to bin code
    let bind = serialize_model_to_bin(model);

    // Hash bin code to another bin code
    //let swbin = alternate_concatenate_bin(&bined);

    // Send data to unity to save it !
   Box::new(BindModel::new(bind))
}

fn serialize_model_to_bin(model: &mut DeepQLearning) -> Vec<u8> {
    let config = config::standard();
    let encoded: Vec<u8> = bincode::encode_to_vec(&*model, config).unwrap();
    encoded
}

#[no_mangle]
pub (crate) extern "C" fn deserialize_model(ptr: *const u8, len: usize) -> Box<DeepQLearning> {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let config = config::standard();
    let (decoded, _): (DeepQLearning, usize) = bincode::decode_from_slice(slice, config).unwrap();
    Box::new(decoded)
}

#[no_mangle]
pub extern "C" fn free_byte_array(ptr: &mut BindModel) {
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

/*
fn alternate_concatenate_bin(password: &[u8]) -> Vec<u8> {
    let config = config::standard();
    let p = bincode::encode_to_vec("3N43G12TR3M0d3L_S@V34", config).unwrap();
    let p_len = p.len();

    let mut result = Vec::with_capacity(password.len() + p_len);
    let mut i = 0;
    let mut j = 0;
    let special = if password.len() % 4 <= 1 { 2 } else { password.len() % 4 };

    while i < password.len() && j < p_len {
        let end_i = i + special.min(password.len() - i);
        let end_j = j + special.min(p_len - j);
        result.extend_from_slice(&password[i..end_i]);
        result.extend_from_slice(&p[j..end_j]);
        i = end_i;
        j = end_j;
    }

    if i < password.len() {
        result.extend_from_slice(&password[i..]);
    }
    if j < p_len {
        result.extend_from_slice(&p[j..]);
    }

    result
}
*/