
use cosmwasm_schema::{cw_serde};
// use cosmwasm_schema::schemars::JsonSchema;
use cosmwasm_schema::serde::{Serialize, Deserialize};
use cosmwasm_std::{Addr, CosmosMsg, StdResult, Timestamp, to_binary, Uint128, WasmMsg};
use cw20::Cw20ReceiveMsg;
use crate::state::{Config, Poll, PollKind, PollStatus, PollVote};

#[cw_serde]
pub struct InstantiateMsg {
    pub voting_token_addr: String,
    pub mixnet_addr: String
}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    CastVote {
        poll_id: u64,
        encrypted_vote: Vec<u8>
    },
    RegisterVoter {
        name: String,
        email: String,
        student_code: u64
    },
    ChangeConfig {
        config: Config
    },
    PushUnmixedVotes {
        poll_id: u64,
        votes: Vec<PollVote>
    },
    ClosePoll {
        poll_id: u64
    }
}

#[cw_serde]
pub enum Cw20HookMsg {
    CreatePoll {
        title: String,
        kind: PollKind,
        description: String,
        start_time: Option<Timestamp>,
        end_time: Timestamp,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Poll {
        poll_id: u64
    },
    // Make status into limit
    Polls {
        status: PollStatus
    },
    VoterInfo {
        addr: String
    },
    IsAdmin{
        addr: String
    },
    PollVotes {
        poll_id: u64
    },
    ParticipatedPolls {
        addr: String
    },
    Config {}
}

