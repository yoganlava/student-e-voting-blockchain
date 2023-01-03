use cosmwasm_schema::{cw_serde};
use cosmwasm_std::Addr;
use cosmwasm_schema::serde::{Serialize, Deserialize};
use crate::state::{PollKind, PollStatus, PollVotes};
use serde_big_array::BigArray;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PollResponse {
    pub creator: Addr,
    pub kind: PollKind,
    pub status: PollStatus,
    pub threshold_percentage: Option<u8>,
    pub start_height: u64,
    pub end_height: Option<u64>,
    pub title: String,
    pub description: String,
    // sent to users?
    #[serde(with = "BigArray")]
    pub public_key: [u8; 65]
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PollVoteCountResponse {
    pub count: PollVotes
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PollsResponse {
    pub polls: Vec<PollResponse>
}

#[cw_serde]
pub struct IsAdminResponse {
    pub is_admin: bool
}
