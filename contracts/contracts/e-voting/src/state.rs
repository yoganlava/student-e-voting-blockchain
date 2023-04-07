use std::fmt;
use std::ops::Add;
use std::str::FromStr;
use cosmwasm_schema::{cw_serde};
use cosmwasm_schema::serde::{Serialize, Deserialize};
use cosmwasm_std::{Addr, BlockInfo, Decimal, Order, StdResult, Storage, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub voting_token_addr: Addr,
    pub admins: Vec<Addr>,
    pub mixnet_addr: Addr
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
    Pending,
    Passed,
    Rejected
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
            "up_vote"    => Ok(VoteKind::UpVote),
            "down_vote"  => Ok(VoteKind::DownVote),
            _           => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PollVotes {
    pub total: u64,
    pub up_votes: u64,
    pub down_votes: u64,
    pub list: Vec<PollVote>,
    pub malformed_votes: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OpaquePollVotes {
    pub total: u64,
    pub up_votes: u64,
    pub down_votes: u64,
    pub malformed_votes: u64
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Poll {
    pub id: u64,
    pub creator: Addr,
    pub kind: PollKind,
    pub status: PollStatus,
    pub threshold_percentage: Option<u8>,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub title: String,
    pub description: String,
    pub votes: PollVotes
}

impl Poll {
    pub fn has_expired(&self, block: &BlockInfo) -> bool {
        block.time >= self.end_time
    }

    pub fn current_status(&self, block: &BlockInfo, storage: &dyn Storage) -> PollStatus {
        if self.has_passed(block, storage) {
            return PollStatus::Passed
        }
        PollStatus::Rejected
    }

    pub fn update_status(&mut self, block: &BlockInfo, storage: &dyn Storage) {

        self.status = self.current_status(block, storage);
    }

    pub fn has_passed(&self, _block: &BlockInfo, storage: &dyn Storage) -> bool {
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

                up_vote_percentage >= percentage_needed
            }
            PollKind::Petition {
              votes_needed
            } => self.votes.up_votes >= votes_needed
        }
    }

    pub fn tally_votes(&mut self) {
       self.votes.list = self.votes.clone().list.iter().map(|item| {
            // TODO make less dirty
            let mut vote = item.to_owned();

            if vote.malformed {
                self.votes.malformed_votes += 1;
                return vote
            }

            let strings: Vec<&str> = vote.decrypted_vote.split('.').collect();
            let decrypted_vote_kind = VoteKind::from_str(strings[0]);
            let decrypted_vote_tracker = u64::from_str(strings[1]);

            if decrypted_vote_kind.is_err() || decrypted_vote_tracker.is_err() {
                vote.malformed = true;
                self.votes.malformed_votes += 1;
                return vote
            }

            match decrypted_vote_kind.clone().unwrap() {
                VoteKind::UpVote => {
                    self.votes.up_votes += 1;
                }
                VoteKind::DownVote => {
                    // TODO CHECK POLL KIND AND IF THE VOTES ARE VALID
                    self.votes.down_votes += 1;
                }
            }


            vote.decrypted_vote_kind = Some(decrypted_vote_kind.unwrap());
            vote.decrypted_vote_tracker = Some(decrypted_vote_tracker.unwrap());

            vote
        }).collect();
    }
}


#[cw_serde]
pub struct PollVote {
    pub voter_addr: Addr,
    pub poll_id: u64,
    pub decrypted_vote_kind: Option<VoteKind>,
    pub decrypted_vote: String,
    pub encrypted_vote: Vec<u8>,
    pub decrypted_vote_tracker: Option<u64>,
    pub malformed: bool
}

#[cw_serde]
pub struct Voter {
    pub addr: Addr,
    pub name: String,
    pub email: String,
    pub student_code: u64,
}

#[cw_serde]
pub struct GiftLog {
    pub receiver: Addr,
    pub amount: Uint128,
    pub message: String
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const POLLS: Map<u64, Poll> = Map::new("polls");
pub const POLL_COUNT: Item<u64> = Item::new("poll_count");
// Votes (voter_addr, poll_id)
// pub const VOTES: Map<(Addr, u64), PollVote> = Map::new("votes");
pub const VOTERS: Map<Addr, Voter> = Map::new("voters");
// Logs of all gifts given by admins
pub const GIFTS: Map<Addr, Vec<GiftLog>> = Map::new("gifts");

pub fn next_poll_id(storage: &mut dyn Storage) -> StdResult<u64> {
    let id: u64 = POLL_COUNT.may_load(storage)?.unwrap_or_default() + 1;
    POLL_COUNT.save(storage, &id)?;
    Ok(id)
}

pub fn register_voter(storage: &mut dyn Storage, addr: &Addr, voter: &Voter) {
    VOTERS.save(storage, addr.clone(), voter).unwrap();
}