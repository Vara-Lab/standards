use sails_rs::prelude::*;

#[derive(Default)]
pub struct ReceiverState {
    pub string_value: String,
    pub num_value: u64,
    pub vec_string: Vec<String>,
    pub callers: Vec<ActorId>
}