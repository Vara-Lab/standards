use sails_rs::{
    calls::{Call, Query}, 
    cell::{
        Ref, RefMut
    }, 
    prelude::*
};

use crate::clients::app_client::traits::QuerySvc as ReceiverQuery;
use crate::states::transmiter_state::TransmitterState;

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum QueryEvents {
    ReceiverContractIdChanged,
    StringValueFromContract(String),
    NumValueFromContract(u64),
    VecStringValueFromContract(Vec<String>),
    VecActorIdValueFromContract(Vec<ActorId>),
    ErrorInReceiverContractResponse,
    ReceiverContractIdNotSpecified
}

pub struct QueryService<'a, ReceiverQueryClient> {
    pub state: Ref<'a, TransmitterState>,
    pub receiver_conotract_query: RefMut<'a, ReceiverQueryClient>
}

#[service]
impl<'a, ReceiverQueryClient> QueryService<'a, ReceiverQueryClient>
where 
    ReceiverQueryClient: ReceiverQuery
{
    pub fn new(
        state: Ref<'a, TransmitterState>, 
        receiver_conotract_query: RefMut<'a, ReceiverQueryClient>
    ) -> Self {
        Self {
            state,
            receiver_conotract_query
        }
    }

    pub fn contract_owner(&self) -> ActorId {
        self.state.owner.to_owned()
    }

    pub fn receiver_id(&self) -> Option<ActorId> {
        self.state.receiver_id.to_owned()
    }

    pub async fn string_value_from_receiver_contract_state(&self) -> QueryEvents {
        if self.state.receiver_id.is_none() {
            return QueryEvents::ReceiverContractIdNotSpecified;
        }

        let receiver_contract_id = self.state.receiver_id.as_ref().unwrap().clone();

        let response_from_contract = self.receiver_conotract_query
            .state_string_value()
            .recv(receiver_contract_id)
            .await;

        let Ok(state_string_value) = response_from_contract else {
            return QueryEvents::ErrorInReceiverContractResponse;
        };

        QueryEvents::StringValueFromContract(state_string_value)
    }

    pub async fn num_value_from_receiver_contract_state(&self) -> QueryEvents {
        if self.state.receiver_id.is_none() {
            return QueryEvents::ReceiverContractIdNotSpecified;
        }

        let receiver_contract_id = self.state.receiver_id.as_ref().unwrap().clone();

        let response_from_contract = self.receiver_conotract_query
            .state_num_value()
            .recv(receiver_contract_id)
            .await;

        let Ok(state_num_value) = response_from_contract else {
            return QueryEvents::ErrorInReceiverContractResponse;
        };

        QueryEvents::NumValueFromContract(state_num_value)
    }

    pub async fn vec_string_value_from_receiver_contract_state(&self) -> QueryEvents {
        if self.state.receiver_id.is_none() {
            return QueryEvents::ReceiverContractIdNotSpecified;
        }

        let receiver_contract_id = self.state.receiver_id.as_ref().unwrap().clone();

        let response_from_contract = self.receiver_conotract_query
            .vec_string_value()
            .recv(receiver_contract_id)
            .await;

        let Ok(state_vec_string_value) = response_from_contract else {
            return QueryEvents::ErrorInReceiverContractResponse;
        };

        QueryEvents::VecStringValueFromContract(state_vec_string_value)
    }

    pub async fn all_callers_from_receiver_contract_state(&self) -> QueryEvents {
        if self.state.receiver_id.is_none() {
            return QueryEvents::ReceiverContractIdNotSpecified;
        }

        let receiver_contract_id = self.state.receiver_id.as_ref().unwrap().clone();

        let response_from_contract = self.receiver_conotract_query
            .all_callers()
            .recv(receiver_contract_id)
            .await;

        let Ok(state_actorid_vec_value) = response_from_contract else {
            return QueryEvents::ErrorInReceiverContractResponse;
        };

        QueryEvents::VecActorIdValueFromContract(state_actorid_vec_value)
    }
}