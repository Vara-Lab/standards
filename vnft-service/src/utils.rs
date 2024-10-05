use core::fmt::Debug;
use gstd::ext;
use sails_rs::collections::{HashMap, HashSet};
use sails_rs::prelude::*;

pub type TokenId = U256;
pub type ApprovalsMap = HashMap<TokenId, ActorId>;
pub type OwnerByIdMap = HashMap<TokenId, ActorId>;
pub type TokensForOwnerMap = HashMap<ActorId, HashSet<TokenId>>;
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
pub enum Error {
    SelfDealing,
    TokenDoesNotExist,
    DeniedAccess,
    NoTokens,
}

pub fn panicking<T, E: Debug, F: FnOnce() -> Result<T, E>>(f: F) -> T {
    match f() {
        Ok(v) => v,
        Err(e) => panic(e),
    }
}

pub fn panic(err: impl Debug) -> ! {
    ext::panic(&format!("{err:?}"))
}
