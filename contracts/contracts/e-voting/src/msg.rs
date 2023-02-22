use cosmwasm_schema::{cw_serde};
use crate::state::{Config, PollKind, PollStatus};

#[cw_serde]
pub struct InstantiateMsg {
    pub voting_token_addr: String,
    pub mixnet_addr: String
}

// Can either Pass or Reject a poll
#[cw_serde]
pub enum ClosePollKind {
    Passed,
    Rejected
}

impl ToString for ClosePollKind {
    fn to_string(&self) -> String {
        match self {
            ClosePollKind::Passed => {
                "Passed".to_string()
            }
            ClosePollKind::Rejected => {
                "Rejected".to_string()
            }
        }
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePoll {
        title: String,
        kind: PollKind,
        description: String,
        start_height: u64,
        end_height: Option<u64>,
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
    // For admins to close certain polls
    ClosePoll {
        poll_id: u64,
        kind: ClosePollKind
    },
    ChangeConfig {
        config: Config
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