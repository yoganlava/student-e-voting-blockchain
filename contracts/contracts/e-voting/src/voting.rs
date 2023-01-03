use std::str::FromStr;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, entry_point, StdResult, Deps, to_binary, Storage, Binary, StdError, BlockInfo, Addr};
use ecies::{PublicKey, SecretKey};
use ecies::utils::generate_keypair;
use rand::Rng;
use crate::error::ContractError;
use crate::msg::{ClosePollKind, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::response::{IsAdminResponse, PollResponse, PollsResponse, PollVoteCountResponse};
use crate::state::{PollVote, Config, CONFIG, Poll, POLLS, PollStatus, VoteKind, VOTERS, VOTES, next_poll_id, PollVotes};
use crate::state::PollStatus::Active;
use crate::utils::{decrypt_message_from_hex, encrypt_message_to_hex};


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = Config {
        voting_token_addr: deps.api.addr_validate(&msg.voting_token_addr)?,
        admins: vec![]
    };

    CONFIG.save(deps.storage, &state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll {
            title,
            kind,
            description,
            start_height,
            end_height
        } => return execute_create_poll(deps.storage, Poll {
            id: 0,
            creator: info.sender,
            kind,
            status: PollStatus::Active,
            threshold_percentage: None,
            start_height,
            end_height,
            title,
            description,
            public_key: [0; 65],
            secret_key: [0; 32],
            votes: PollVotes {
                total: 0,
                up_votes: 0,
                down_votes: 0
            }
        }),
        ExecuteMsg::CastVote {
            poll_id,
            encrypted_vote
        } => return execute_cast_vote(deps.storage, PollVote {
                voter_addr: info.sender,
                poll_id,
                decrypted_vote_kind: None,
                encrypted_vote,
                tracker: None
        },
        &env.block),
        // TODO MAKE AND THEN START FRONTEND
        ExecuteMsg::RegisterVoter { .. } => {}
        // Veto close
        ExecuteMsg::ClosePoll {
            poll_id,
            kind
        } => return execute_close_poll(deps.storage, poll_id, kind, info.sender)
    }
    Ok(Response::default())
}

fn execute_create_poll(storage: &mut dyn Storage, mut poll: Poll) -> Result<Response, ContractError> {
    let (secret_key, public_key) = generate_keypair();
    poll.secret_key = secret_key.serialize();
    poll.public_key = public_key.serialize();

    // TODO verification

    let poll_id = next_poll_id(storage).unwrap();
    poll.id = poll_id;
    POLLS.save(storage, poll_id, &poll)?;

    Ok(Response::new()
        .add_attribute("action", "create_poll")
        .add_attribute("poll_id", poll_id.to_string())
    )
}

fn execute_cast_vote(storage: &mut dyn Storage, vote: PollVote, block: &BlockInfo) -> Result<Response, ContractError> {
    // Does poll exist?
    let mut poll = match POLLS.may_load(storage, vote.poll_id)? {
        Some(poll) => Some(poll),
        None => return Err(ContractError::PollNotExist {}),
    }.unwrap();

    if !VOTERS.has(storage, vote.voter_addr.clone()) {
        return Err(ContractError::VoterNotExist {});
    }

    if !(poll.status == PollStatus::Active) {
        return Err(ContractError::PollNotActive {});
    }

    // check if already voted
    if !VOTES.has(storage, (vote.voter_addr.clone(), vote.poll_id)) {
        return Err(ContractError::AlreadyVoted {});
    }

    let mut rng = rand::thread_rng();
    let tracker = rng.gen::<u64>().to_string();
    let tracker_hex = encrypt_message_to_hex(PublicKey::parse(&poll.public_key).unwrap(), tracker.clone());

    // update poll state
    poll.votes.total += 1;
    poll.update_status(block, storage);
    POLLS.save(storage, poll.id, &poll).unwrap();

    VOTES.save(storage, (vote.voter_addr.clone(), vote.poll_id), &PollVote {
        voter_addr: vote.voter_addr.clone(),
        poll_id: vote.poll_id,
        decrypted_vote_kind: None,
        encrypted_vote: vec![vote.encrypted_vote, tracker_hex].join("."),
        tracker: None
    }).unwrap();

    Ok(Response::new()
           .add_attribute("action", "vote")
           .add_attribute("tracker" ,tracker)
    )
}

fn decrypt_votes(storage: &mut dyn Storage, secret_key: SecretKey, poll_id: u64) -> (u64, u64) {
    let filtered_poll_votes= &VOTES
        .range(storage, None, None, cosmwasm_std::Order::Ascending)
        .filter(|item| {
            let ((_, vote_poll_id) , _) = item.as_ref().unwrap();
            *vote_poll_id == poll_id
        }).collect::<Vec<_>>();
    let mut up_votes = 0u64;
    let mut down_votes = 0u64;
    filtered_poll_votes.iter().for_each(|item| {
        // TODO make less dirty
        let (key, mut vote) = item.as_ref().unwrap().clone();
        let decrypted_vote = decrypt_message_from_hex(secret_key, vote.encrypted_vote.clone());
        let strings: Vec<&str> = decrypted_vote.split(".").collect();
        let decrypted_vote_kind = VoteKind::from_str(strings[0]).unwrap();
        match decrypted_vote_kind {
            VoteKind::UpVote => {
                up_votes += 1;
            }
            VoteKind::DownVote => {
                // TODO CHECK POLL KIND AND IF THE VOTES ARE VALID
                down_votes += 1;
            }
        }
        vote.decrypted_vote_kind = Some(decrypted_vote_kind);
        vote.tracker = Some(u64::from_str(strings[1]).unwrap());

        // save vote
        VOTES.save(storage, key.clone(), &vote).unwrap();
    });
    (up_votes, down_votes)
}

fn is_admin(storage: &dyn Storage, addr: Addr) -> bool {
    CONFIG.load(storage).unwrap().admins.contains(&addr)
}

fn execute_close_poll(storage: &mut dyn Storage, poll_id: u64, kind: ClosePollKind, addr: Addr) -> Result<Response, ContractError> {
    if !is_admin(storage, addr) {
        return Err(ContractError::InvalidAuthorisation {});
    }

    // TODO: check If already passed/closed etc

    let mut poll = match POLLS.may_load(storage, poll_id)? {
        Some(poll) => Some(poll),
        None => return Err(ContractError::PollNotExist {}),
    }.unwrap();

    if poll.status != Active {
        return Err(ContractError::InactivePoll {})
    }

    poll.status = match kind {
        ClosePollKind::Passed => {
            PollStatus::Passed
        }
        ClosePollKind::Rejected => {
            PollStatus::Rejected
        }
    };

    (poll.votes.up_votes, poll.votes.down_votes) = decrypt_votes(storage, SecretKey::parse(&poll.secret_key).unwrap(), poll_id);

    POLLS.save(storage, poll.id, &poll).unwrap();

    Ok(Response::new()
        .add_attribute("action", "close")
        .add_attribute("status", kind.to_string())
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Poll {
            poll_id
        } => query_poll(deps.storage, poll_id),
        QueryMsg::Polls{
            status
        } => query_polls(deps.storage, status),
        QueryMsg::VoterInfo { addr } => to_binary(&VOTERS.load(deps.storage, deps.api.addr_validate(&addr)?)?),
        QueryMsg::IsAdmin {
            addr
        } => to_binary(
            &IsAdminResponse {
                is_admin: is_admin(deps.storage, deps.api.addr_validate(&addr)?)
            }
        ),
        // Query number of votes in poll
        QueryMsg::PollVotes {
            poll_id
        } => query_poll_vote_count(deps.storage, poll_id)
    }
}

fn query_poll(storage: &dyn Storage, poll_id: u64) -> StdResult<Binary> {
    let poll = match POLLS.may_load(storage, poll_id)? {
        Some(poll) => Some(poll),
        None => return Err(StdError::generic_err("Poll does not exist")),
    }
    .unwrap();

    to_binary(&PollResponse {
        creator: poll.creator,
        kind: poll.kind,
        status: poll.status,
        threshold_percentage: poll.threshold_percentage,
        start_height: poll.start_height,
        end_height: poll.end_height,
        title: poll.title,
        description: poll.description,
        public_key: poll.public_key
    })
}

fn query_polls(storage: &dyn Storage, status: PollStatus) -> StdResult<Binary> {
    to_binary(
        &PollsResponse {
            polls: POLLS
                .range(storage, None, None, cosmwasm_std::Order::Ascending)
                .filter(|item| {
                    let (_, poll) = item.as_ref().unwrap();
                    poll.status == status
                }
                )
                .map(|item| {
                    let (_, poll) = item.unwrap();
                    PollResponse {
                        creator: poll.creator,
                        kind: poll.kind,
                        status: poll.status,
                        threshold_percentage: poll.threshold_percentage,
                        start_height: poll.start_height,
                        end_height: poll.end_height,
                        title: poll.title,
                        description: poll.description,
                        public_key: poll.public_key
                    }
                })
                .collect::<Vec<PollResponse>>()
        }
    )
}

fn query_poll_vote_count(storage: &dyn Storage, poll_id: u64) -> StdResult<Binary> {
    to_binary(
        &PollVoteCountResponse {
            count: POLLS.load(storage, poll_id)?.votes
        }
    )
}