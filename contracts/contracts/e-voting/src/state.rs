use std::fmt;
use std::ops::{Add, Div};
use std::str::FromStr;
use cosmwasm_schema::{cw_serde};
use cosmwasm_schema::serde::{Serialize, Deserialize};
use cosmwasm_std::{Addr, BlockInfo, Decimal, Order, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};
use serde_big_array::BigArray;

#[cw_serde]
pub struct Config {
    pub voting_token_addr: Addr,
    pub admins: Vec<Addr>,
    // pub owner: Vec<Addr>,
    // pub poll_count: u64,
    // pub staked_tokens: Uint128
}

#[cw_serde]
pub enum PollKind {
    Threshold {
        votes_needed: u64
    },
    Percentage {
        percentage_needed: Decimal
    },
    Petition {
        votes_needed: u64
    }
}

#[cw_serde]
pub enum PollStatus {
    Active,
    Passed,
    Rejected,
    Canceled
}

#[cw_serde]
pub enum VoteKind {
    UpVote,
    DownVote
}

impl fmt::Display for VoteKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for VoteKind {

    type Err = ();

    fn from_str(input: &str) -> Result<VoteKind, Self::Err> {
        match input {
            "UpVote"    => Ok(VoteKind::UpVote),
            "DownVote"  => Ok(VoteKind::DownVote),
            _           => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PollVotes {
    pub total: u64,
    pub up_votes: u64,
    pub down_votes: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Poll {
    pub id: u64,
    pub creator: Addr,
    pub kind: PollKind,
    pub status: PollStatus,
    pub threshold_percentage: Option<u8>,
    pub start_height: u64,
    pub end_height: Option<u64>,
    pub title: String,
    pub description: String,
    // sent to users to encrypt vote
    #[serde(with = "BigArray")]
    pub public_key: [u8; 65],
    // used to decrypt vote
    pub secret_key: [u8; 32],
    pub votes: PollVotes
}

impl Poll {
    pub fn current_status(&self, block: &BlockInfo, storage: &dyn Storage) -> PollStatus {
        if self.status == PollStatus::Active {
            if self.has_passed(block, storage) {
                return PollStatus::Passed
            }
            if self.has_expired(block) {
                return PollStatus::Rejected
            }
        }

        self.status.clone()
    }

    pub fn has_expired(&self, block: &BlockInfo) -> bool {
        self.end_height.unwrap_or_default() >= block.height
    }

    pub fn update_status(&mut self, block: &BlockInfo, storage: &dyn Storage) {
        self.status = self.current_status(block, storage);
    }

    pub fn has_passed(&self, block: &BlockInfo, storage: &dyn Storage) -> bool {
        match self.kind {
            PollKind::Threshold {
                votes_needed
            } => self.votes.up_votes >= votes_needed,
            PollKind::Percentage {
                percentage_needed
            } => {
                let voter_count = VOTERS.range(storage, None, None, Order::Descending).count();

                let up_vote_percentage = Decimal::from_ratio(
                    Uint128::from(self.votes.up_votes),
                    Uint128::from(self.votes.up_votes)
                        .add(Uint128::from(voter_count as u64))
                );

                return up_vote_percentage >= percentage_needed;
            }
            // TODO: Maybe add Considering for PollStatus?
            PollKind::Petition {
              votes_needed
            } => self.votes.up_votes >= votes_needed
        };
        false
    }
}


#[cw_serde]
pub struct PollVote {
    pub voter_addr: Addr,
    pub poll_id: u64,
    pub decrypted_vote_kind: Option<VoteKind>,
    pub encrypted_vote: String,
    pub tracker: Option<u64>
}

#[cw_serde]
pub struct Voter {
    addr: Addr,
    name: String,
    email: String,
    student_code: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const POLLS: Map<u64, Poll> = Map::new("polls");
pub const POLL_COUNT: Item<u64> = Item::new("poll_count");
// Votes (voter_addr, poll_id)
pub const VOTES: Map<(Addr, u64), PollVote> = Map::new("votes");
pub const VOTERS: Map<Addr, Voter> = Map::new("voters");

pub fn next_poll_id(storage: &mut dyn Storage) -> StdResult<u64> {
    let id: u64 = POLL_COUNT.may_load(storage)?.unwrap_or_default() + 1;
    POLL_COUNT.save(storage, &id)?;
    Ok(id)
}

pub fn register_voter(storage: &mut dyn Storage, addr: &Addr, voter: &Voter) {
    VOTERS.save(storage, addr.clone(), voter).unwrap();
}
//     pub creator: Addr,
//     pub status: PollStatus,
//     pub quorum_percentage: Option<u8>,
//     pub yes_votes: Uint128,
//     pub no_votes: Uint128,
//     pub voters: Vec<Addr>,
//     pub voter_info: Vec<Voter>,
//     pub end_height: u64,
//     pub start_height: Option<u64>,
//     pub description: String,
// }

//
// #[cw_serde]
// #[derive(Default)]
// pub struct TokenManager {
//     pub token_balance: Uint128,             // total staked balance
//     pub locked_tokens: Vec<(u64, Uint128)>, //maps poll_id to weight voted
//     pub participated_polls: Vec<u64>,       // poll_id
// }
//
// #[cw_serde]
// pub struct Voter {
//     pub vote: String,
//     pub weight: Uint128,
// }
//
// #[cw_serde]
// pub enum PollStatus {
//     InProgress,
//     Tally,
//     Passed,
//     Rejected,
// }
//

//

// pub const BANK: Map<&[u8], TokenManager> = Map::new("bank");
