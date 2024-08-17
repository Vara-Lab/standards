use sails_rs::prelude::*;

pub struct TransmitterState {
    pub receiver_id: Option<ActorId>,
    pub owner: ActorId
}