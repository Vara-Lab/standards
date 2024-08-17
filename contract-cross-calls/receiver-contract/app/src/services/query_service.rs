use sails_rs::{
    prelude::*,
    cell::RefCell
};

use crate::states::receiver_state::ReceiverState;

pub struct QueryService<'a> {
    receiver_state: &'a RefCell<ReceiverState>
}

#[service]
impl<'a> QueryService<'a> {
    pub fn new(receiver_state: &'a RefCell<ReceiverState>) -> Self {
        Self {
            receiver_state
        }
    }

    pub fn state_string_value(&self) -> String {
        let state = self.receiver_state.take();

        state.string_value
    }

    pub fn state_num_value(&self) -> u64 {
        let state = self.receiver_state.take();

        state.num_value
    }

    pub fn vec_string_value(&self) -> Vec<String> {
        let state = self.receiver_state.take();

        state.vec_string
    }

    pub fn all_callers(&self) -> Vec<ActorId> {
        let state = self.receiver_state.take();

        state.callers
    }
}

