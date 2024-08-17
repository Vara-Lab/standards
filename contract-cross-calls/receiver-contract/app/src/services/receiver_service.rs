use sails_rs::{
    prelude::*,
    cell::RefMut,
    gstd::msg
};

use crate::states::receiver_state::ReceiverState;

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ReceiverEvents {
    StringValueChanged {
        new: String,
        old: String,
    },
    NumValueChange {
        new: u64,
        old: u64
    },
    StringAddedToVec {
        added: String
    }
}

pub struct ReceiverService<'a> {
    state: RefMut<'a, ReceiverState>
}

#[service]
impl<'a> ReceiverService<'a> {
    pub fn new(state: RefMut<'a, ReceiverState>) -> Self {
        Self {
            state
        }
    }

    pub fn set_string_value(&mut self, new_value: String) -> ReceiverEvents {
        let caller = msg::source();
        let response = ReceiverEvents::StringValueChanged { 
            new: new_value.clone(), 
            old: self.state.string_value.clone() 
        };

        self.state.string_value = new_value.clone();
        self.state.callers.push(caller);

        response
    } 

    pub fn set_num_value(&mut self, new_value: u64) -> ReceiverEvents {
        let caller = msg::source();
        let response = ReceiverEvents::NumValueChange { 
            new: self.state.num_value, 
            old: new_value
        };

        self.state.num_value = new_value;
        self.state.callers.push(caller);

        response
    }

    pub fn add_string_to_vec(&mut self, value: String) -> ReceiverEvents {
        let caller = msg::source();
        let response = ReceiverEvents::StringAddedToVec { 
            added: value.clone() 
        };

        self.state.vec_string.push(value);
        self.state.callers.push(caller);

        response
    }
}