use cosmwasm_schema::{cw_serde};
use cosmwasm_std::{Addr, Timestamp};
use cosmwasm_schema::serde::{Serialize, Deserialize};
use crate::state::{OpaquePollVotes, PollKind, PollStatus, PollVotes};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PollResponse {
    pub id: u64,
    pub creator: Addr,
    pub kind: PollKind,
    pub status: PollStatus,
    pub threshold_percentage: Option<u8>,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub title: String,
    pub description: String,
    pub votes: OpaquePollVotes
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
