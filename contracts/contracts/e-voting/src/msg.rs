use std::fmt;
use cosmwasm_schema::{cw_serde};
use cosmwasm_schema::schemars::JsonSchema;
// use cosmwasm_schema::schemars::JsonSchema;

use cosmwasm_std::{Addr, CosmosMsg, StdResult, Timestamp, to_binary, WasmMsg};
use cw20::Cw20ReceiveMsg;
use crate::state::{Config, GiftLog, PollKind, PollStatus, PollVote};

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
    },
    Callback(CallbackMsg)
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
    GiftVoter {
        receiver: Addr,
        message: String
    }
}

#[cw_serde]
pub enum CallbackMsg {
    AfterGiftVoter {
        gift_log: GiftLog
    }
}

impl CallbackMsg {
    pub fn to_cosmos_msg<T: Clone + fmt::Debug + PartialEq + JsonSchema>(
        self,
        contract_addr: &Addr,
    ) -> StdResult<CosmosMsg<T>> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: String::from(contract_addr),
            msg: to_binary(&ExecuteMsg::Callback(self))?,
            funds: vec![],
        }))
    }
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
    Config {},
    Gifts {
        addr: String
    },
    Vote {
        addr: String,
        poll_id: u64
    }
}

