#![no_std]

use sails_rs::{
    prelude::*,
    cell::RefCell
};

pub mod services;
pub mod states;

use services::{
    receiver_service::ReceiverService,
    query_service::QueryService
};
use states::receiver_state::ReceiverState;

pub struct ReceiverProgram {
    receiver_state: RefCell<ReceiverState>
}

#[program]
impl ReceiverProgram {
    pub fn new() -> Self {
        let receiver_state = RefCell::new(
            ReceiverState::default()
        );
        
        Self {
            receiver_state
        }
    }

    #[route("Receiver")]
    pub fn receiver_svc(&self) -> ReceiverService {
        ReceiverService::new(
            self.receiver_state.borrow_mut()
        )
    }

    #[route("QuerySVC")]
    pub fn query_svc(&self) -> QueryService {
        QueryService::new(&self.receiver_state)
    }
}