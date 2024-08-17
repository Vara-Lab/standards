use sails_rs::{
    calls::{Call, Query}, 
    cell::{
        Ref, RefMut
    }, 
    prelude::*
};

use crate::clients::app_client::{
    traits::{
        Receiver, 
        QuerySvc as ReceiverQuery
    },
    ReceiverEvents
};

use crate::states::transmiter_state::TransmitterState;

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum TransmitterEvents {
    ReceiverContractIdChanged,
    StringValueChanged {
        new: String,
        old: String
    },
    NumValueChanged {
        new: u64,
        old: u64
    },
    StringValueAdded {
        value_added: String
    },
    StringValueFromContract(String),
    NumValueFromContract(u64),
    VecStringValueFromContract(Vec<String>),
    ReceiverContractIdNotSpecified,
    ErrorInReceiverContractResponse,
    IncorrectAnswerFromReceiverContract
}

pub struct TransmitterService<'a, ReceiverClient, ReceiverQueryClient> {
    pub state: RefMut<'a, TransmitterState>,
    pub receiver_contract_service: RefMut<'a, ReceiverClient>,
    pub receiver_conotract_query: RefMut<'a, ReceiverQueryClient>
}

#[service]
impl<'a, ReceiverClient, ReceiverQueryClient> TransmitterService<'a, ReceiverClient, ReceiverQueryClient>
where 
    ReceiverClient: Receiver,
    ReceiverQueryClient: ReceiverQuery
{
    pub fn new(
        state: RefMut<'a, TransmitterState>, 
        receiver_contract_service: RefMut<'a, ReceiverClient>,
        receiver_conotract_query: RefMut<'a, ReceiverQueryClient>
    ) -> Self {
        Self {
            state,
            receiver_contract_service,
            receiver_conotract_query
        }
    }

    pub fn set_receiver_contract_id(&mut self, receiver_id: ActorId) -> TransmitterEvents {
        // self.state.insert(receiver_id);
        let _ = self
            .state
            .receiver_id
            .insert(receiver_id);

        TransmitterEvents::ReceiverContractIdChanged
    }

    pub async fn change_receiver_contract_string_value(&mut self, new_val: String) -> TransmitterEvents {
        if self.state.receiver_id.is_none() {
            return TransmitterEvents::ReceiverContractIdNotSpecified;
        }

        let receiver_contract_id = self.state.receiver_id.as_ref().unwrap().clone();

        let response_from_contract = self.receiver_contract_service
            .set_string_value(new_val)
            .send_recv(receiver_contract_id)
            .await;

        let Ok(receiver_message) = response_from_contract else {
            return TransmitterEvents::ErrorInReceiverContractResponse;
        };
        
        let ReceiverEvents::StringValueChanged { new, old } = receiver_message else {
            return TransmitterEvents::IncorrectAnswerFromReceiverContract;
        };

        TransmitterEvents::StringValueChanged { 
            new, 
            old 
        }
    }

    pub async fn string_value_from_receiver_contract(&mut self) -> TransmitterEvents {
        if self.state.receiver_id.is_none() {
            return TransmitterEvents::ReceiverContractIdNotSpecified;
        }

        let receiver_contract_id = self.state.receiver_id.as_ref().unwrap().clone();

        let response_from_contract = self.receiver_conotract_query
            .state_string_value()
            .recv(receiver_contract_id)
            .await;

        let Ok(receiver_state_value) = response_from_contract else {
            return TransmitterEvents::ErrorInReceiverContractResponse;
        };

        TransmitterEvents::StringValueFromContract(receiver_state_value)
    }

    pub async fn change_receiver_contract_num_value(&mut self, new_val: u64) -> TransmitterEvents {
        if self.state.receiver_id.is_none() {
            return TransmitterEvents::ReceiverContractIdNotSpecified;
        }

        let receiver_contract_id = self.state.receiver_id.as_ref().unwrap().clone();

        let response_from_contract = self.receiver_contract_service
            .set_num_value(new_val)
            .send_recv(receiver_contract_id)
            .await;

        let Ok(receiver_message) = response_from_contract else {
            return TransmitterEvents::ErrorInReceiverContractResponse;
        };

        let ReceiverEvents::NumValueChange { new, old } = receiver_message else {
            return TransmitterEvents::IncorrectAnswerFromReceiverContract;
        };

        TransmitterEvents::NumValueChanged { 
            new, 
            old 
        }
    }

    pub async fn num_value_from_receiver_contract(&mut self) -> TransmitterEvents {
        if self.state.receiver_id.is_none() {
            return TransmitterEvents::ReceiverContractIdNotSpecified;
        }

        let receiver_contract_id = self.state.receiver_id.as_ref().unwrap().clone();

        let response_from_contract = self.receiver_conotract_query
            .state_num_value()
            .recv(receiver_contract_id)
            .await;

        let Ok(receiver_state_value) = response_from_contract else {
            return TransmitterEvents::ErrorInReceiverContractResponse;
        };

        TransmitterEvents::NumValueFromContract(receiver_state_value)
    }

    pub async fn add_string_to_receiver_contract_vec_string(&mut self, value: String) -> TransmitterEvents {
        if self.state.receiver_id.is_none() {
            return TransmitterEvents::ReceiverContractIdNotSpecified;
        }

        let receiver_contract_id = self.state.receiver_id.as_ref().unwrap().clone();

        let response_from_contract = self.receiver_contract_service
            .add_string_to_vec(value)
            .send_recv(receiver_contract_id)
            .await;

        let Ok(receiver_message) = response_from_contract else {
            return TransmitterEvents::ErrorInReceiverContractResponse;
        };

        let ReceiverEvents::StringAddedToVec { added: value_added } = receiver_message else {
            return TransmitterEvents::IncorrectAnswerFromReceiverContract;
        };

        TransmitterEvents::StringValueAdded { 
            value_added 
        }
    }

    pub async fn vec_string_from_receiver_contract(&mut self) -> TransmitterEvents {
        if self.state.receiver_id.is_none() {
            return TransmitterEvents::ReceiverContractIdNotSpecified;
        }

        let receiver_contract_id = self.state.receiver_id.as_ref().unwrap().clone();

        let response_from_contract = self.receiver_conotract_query
            .vec_string_value()
            .recv(receiver_contract_id)
            .await;

        let Ok(receiver_message) = response_from_contract else {
            return TransmitterEvents::ErrorInReceiverContractResponse;
        };

        TransmitterEvents::VecStringValueFromContract(receiver_message)
    }
}