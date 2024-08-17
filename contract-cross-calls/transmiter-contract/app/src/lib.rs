#![no_std]

use sails_rs::{
    prelude::*,
    cell::RefCell,
    gstd::{
        calls::GStdRemoting,
        msg
    }
};

pub mod services;
pub mod clients;
pub mod states;

use services::{
    transmitter_service::TransmitterService,
    query_service::QueryService
};
use clients::app_client::{
    Receiver as ReceiverClient,
    QuerySvc as ReceiverQueryClient
};

use states::transmiter_state::TransmitterState;

pub struct TransmitterProgram {
    pub transmiter_state: RefCell<TransmitterState>,
    pub receiver_client: RefCell<ReceiverClient<GStdRemoting>>,
    pub receiver_query_client: RefCell<ReceiverQueryClient<GStdRemoting>>
}

#[program]
impl TransmitterProgram {
    pub fn new() -> Self {
        let transmiter_state = RefCell::new(
            TransmitterState {
                receiver_id: None,
                owner: msg::source()
            }
        );
        let receiver_client = RefCell::new(
            ReceiverClient::new(GStdRemoting)
        );
        let receiver_query_client = RefCell::new(
            ReceiverQueryClient::new(GStdRemoting)
        );

        Self {
            transmiter_state,
            receiver_client,
            receiver_query_client
        }
    }

    pub fn new_with_receiver_id(receiver_id: ActorId) -> Self {
        let transmiter_state = RefCell::new(
            TransmitterState {
                receiver_id: Some(receiver_id),
                owner: msg::source()
            }
        );

        let receiver_client = RefCell::new(
            ReceiverClient::new(GStdRemoting)
        );
        let receiver_query_client = RefCell::new(
            ReceiverQueryClient::new(GStdRemoting)
        );

        Self {
            transmiter_state,
            receiver_client,
            receiver_query_client
        }
    }

    #[route("Transmitter")]
    pub fn transmitter_svc(&self) -> TransmitterService<'_, ReceiverClient<GStdRemoting>, ReceiverQueryClient<GStdRemoting>> {
        TransmitterService::new(
            self.transmiter_state.borrow_mut(), 
            self.receiver_client.borrow_mut(), 
            self.receiver_query_client.borrow_mut()
        )
    }

    
    #[route("QueryService")]
    pub fn query_svc(&self) -> QueryService<'_, ReceiverQueryClient<GStdRemoting>> {
        QueryService::new(
            self.transmiter_state.borrow(), 
            self.receiver_query_client.borrow_mut()
        )

    }

}

