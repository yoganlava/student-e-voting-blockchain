use cosmwasm_schema::{cw_serde};
use cosmwasm_std::Timestamp;
use crate::state::{Config, PollKind, PollStatus, PollVote, PollVotes};

#[cw_serde]
pub struct InstantiateMsg {
    pub voting_token_addr: String,
    pub mixnet_addr: String
}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePoll {
        title: String,
        kind: PollKind,
        description: String,
        start_time: Option<Timestamp>,
        end_time: Timestamp,
    },
    CastVote {
        poll_id: u64,
        encrypted_vote: String
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
pub enum QueryMsg {
    Poll {
        poll_id: u64
    },
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
    }
}