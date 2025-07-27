#[cfg(test)]
mod tests {
    use crate::deep_reinforce_learning::deep_q_learning::create::init_dql;
    use crate::deep_reinforce_learning::deep_q_learning::dql_struct::DeepQLearning;
    use crate::deep_reinforce_learning::deep_q_learning::manager::{check_size, deserialize_model, save_model};
    use crate::deep_reinforce_learning::deep_q_learning::predict::choose_action;
    use crate::deep_reinforce_learning::deep_q_learning::train::{learn_dql, update_epsilon};

    fn setup_model() -> Box<DeepQLearning> {
        // Init Data
        let model = init_dql(
            10,
            10,
            0.001,
            0.95,
            1.0,
            0.01,
            0.995
        );
        model
    }

    #[test]
    fn is_init() {
        let model = setup_model();
        assert_eq!(model.state_size, 10);
        assert_eq!(model.action_size, 10);
        assert_eq!(model.self_ptr, 0);
    }

    #[test]
    fn is_ca() {
        let mut model = setup_model();
        let slice= [1.0f32, 2.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32, 7.0f32, 8.0f32, 9.0f32, 10.0f32];
        let a = [0, 1, 3];
        let val = choose_action(&mut model, slice.as_ptr(), slice.len() as i32, 10, a.as_ptr(), a.len() as i32);
        assert!(a.contains(&val), "The array does not contain the value");
    }

    #[test]
    fn is_update_epsi() {
        let mut model = setup_model();
        let old_epsi = model.epsilon.clone();
        update_epsilon(&mut *model);
        let curr_epsi = model.epsilon.clone();
        assert_ne!(old_epsi, curr_epsi);
    }

    #[test]
    fn is_train() {
        let mut model = setup_model();

        let initial_nn = (*model.neural_network).clone();

        let state= [1.0f32, 2.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32, 7.0f32, 8.0f32, 9.0f32, 10.0f32];
        let next_state = [2.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32, 7.0f32, 8.0f32, 9.0f32, 10.0f32, 11.0f32];

        learn_dql(&mut model, state.as_ptr(), state.len() as i32, 0, 1, next_state.as_ptr(), next_state.len() as i32, 10, false);

        let nn = model.neural_network.clone();
        let cur = (*nn).clone();
        assert_ne!(cur.weights[1][0], initial_nn.weights[1][0]);

    }

    #[test]
    fn is_prediction() {
        let mut model = setup_model();

        let state= [1.0f32, 2.0f32, 3.0f32, 4.0f32, 5.0f32, 6.0f32, 7.0f32, 8.0f32, 9.0f32, 10.0f32];
        let available_action = [0, 1, 2, 3];

        let action = choose_action(&mut *model, state.as_ptr(), state.len() as i32, 10, available_action.as_ptr(), available_action.len() as i32);

        println!("{:?}", action);
        assert!(available_action.contains(&action));
    }

    #[test]
    fn is_check_size() {
        let mut model = setup_model();
        let size = check_size(&mut model);
        println!("{:?} ", size);
        assert_ne!(size, 0);
    }

    /*#[test]
    fn is_bin() {
        let mut model = setup_model();
        let (ptr, len) = save_model(&mut model);
        let slice = unsafe {Vec::from_raw_parts(ptr, len, len)};
        println!("{:?}", slice);

        let res = slice.len();
        assert_ne!(res, 0);
    }

    #[test]
    fn is_deserialize() {
        let mut model = setup_model();
        let (ptr, len) = save_model(&mut model);
        let _ = unsafe {Vec::from_raw_parts(ptr, len, len)};

        let new_model = Box::new(deserialize_model(ptr, len));
        assert_eq!(new_model, model);
    }*/
}