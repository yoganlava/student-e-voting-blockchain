use crate::error::ContractError;
use crate::msg::{ClosePollKind, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::response::{IsAdminResponse, PollResponse, PollVoteCountResponse, PollsResponse};
use crate::state::PollStatus::{Active, Pending};
use crate::state::{
    next_poll_id, Config, Poll, PollStatus, PollVote, PollVotes, VoteKind, Voter, CONFIG, POLLS,
    VOTERS,
};
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, BlockInfo, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdError, StdResult, Storage,
};
use std::str::FromStr;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = Config {
        voting_token_addr: deps.api.addr_validate(&msg.voting_token_addr)?,
        admins: vec![
            info.sender
        ],
        mixnet_addr: deps.api.addr_validate(&msg.mixnet_addr)?
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
            end_height,
        } => {
            return execute_create_poll(
                deps.storage,
                Poll {
                    id: 0,
                    creator: info.sender,
                    kind,
                    status: PollStatus::Active,
                    threshold_percentage: None,
                    start_height,
                    end_height,
                    title,
                    description,
                    votes: PollVotes {
                        total: 0,
                        up_votes: 0,
                        down_votes: 0,
                        list: vec![]
                    }
                }
            )
        }
        ExecuteMsg::CastVote {
            poll_id,
            encrypted_vote,
        } => {
            return execute_cast_vote(
                deps.storage,
                PollVote {
                    voter_addr: info.sender,
                    poll_id,
                    decrypted_vote_kind: None,
                    encrypted_vote,
                    tracker: None,
                },
                &env.block,
            )
        }
        // TODO MAKE AND THEN START FRONTEND
        ExecuteMsg::RegisterVoter {
            name,
            email,
            student_code,
        } => {
            return execute_register_voter(deps.storage, Voter {
                name,
                email,
                student_code,
                addr: info.sender
            })
        }
        // Veto close
        ExecuteMsg::ClosePoll { poll_id, kind } => {
            return execute_close_poll(deps.storage, poll_id, kind, env.block,  info.sender)
        }
        ExecuteMsg::ChangeConfig { .. } => {}
    }
    Ok(Response::default())
}

fn execute_register_voter(
    storage: &mut dyn Storage,
    voter: Voter,
) -> Result<Response, ContractError> {
    if VOTERS.has(storage, voter.addr.clone()) {
        return Err(ContractError::VoterAlreadyExist {});
    }

    // Check if email/code already been added
    let res = VOTERS
        .range(storage, None, None, Order::Ascending)
        .try_for_each(|item| {
            let (_, current_voter) = item.unwrap();
            if current_voter.email == voter.email || current_voter.student_code == voter.student_code {
                return Err(ContractError::VoterAlreadyExist {});
            }
            Ok(())
        });

    if res.is_err() {
        return Err(ContractError::VoterAlreadyExist {});
    }

    VOTERS.save(storage, voter.addr.clone(), &voter)?;

    Ok(Response::new().add_attribute("action", "register_voter"))
}

fn execute_change_config(
    storage: &mut dyn Storage,
    config: Config,
    addr: Addr,
) -> Result<Response, ContractError> {
    let current_config = CONFIG.load(storage).unwrap();
    if !current_config.admins.contains(&addr) {
        return Err(ContractError::InvalidAuthorisation {});
    }

    CONFIG.save(storage, &config).unwrap();

    Ok(Response::default())
}

fn execute_create_poll(
    storage: &mut dyn Storage,
    mut poll: Poll,
) -> Result<Response, ContractError> {
    // TODO verification

    let poll_id = next_poll_id(storage)?;
    poll.id = poll_id;
    POLLS.save(storage, poll_id, &poll)?;

    Ok(Response::new()
        .add_attribute("action", "create_poll")
        .add_attribute("poll_id", poll_id.to_string()))
}

fn execute_cast_vote(
    storage: &mut dyn Storage,
    vote: PollVote,
    block: &BlockInfo,
) -> Result<Response, ContractError> {
    // Does poll exist?
    let mut poll = match POLLS.may_load(storage, vote.poll_id)? {
        Some(poll) => Some(poll),
        None => return Err(ContractError::PollNotExist {}),
    }
    .unwrap();

    if !VOTERS.has(storage, vote.voter_addr.clone()) {
        return Err(ContractError::VoterNotExist {});
    }

    if poll.status != PollStatus::Active {
        return Err(ContractError::PollNotActive {});
    }

    if poll.has_expired(block) {
        poll.status = PollStatus::Pending;
        POLLS.save(storage, poll.id, &poll).unwrap();
        return Err(ContractError::PollNotActive {});
    }

    // check if encryption is correct??

    // check if already voted
    if poll.votes.list.iter().any(|v| v.voter_addr == vote.voter_addr) {
        return Err(ContractError::AlreadyVoted {});
    }


    // add vote to poll
    poll.votes.total += 1;
    poll.votes.list.push(PollVote {
        voter_addr: vote.voter_addr.clone(),
        poll_id: vote.poll_id,
        decrypted_vote_kind: None,
        encrypted_vote: vote.encrypted_vote,
        tracker: None,
    });

    POLLS.save(storage, poll.id, &poll).unwrap();

    Ok(Response::new()
        .add_attribute("action", "vote")
        .add_attribute("tracker", tracker))
}

fn execute_push_unmixed_votes(storage: &mut dyn Storage, poll_id: u64, votes: Vec<PollVote>) -> Result<Response, ContractError> {
    let mut poll = match POLLS.may_load(storage, poll_id)? {
        Some(poll) => Some(poll),
        None => return Err(ContractError::PollNotExist {}),
    }.unwrap();

    if e

    poll.votes.list = votes;
    POLLS.save(storage, poll.id, &poll).unwrap();

    Ok(Response::new()
        .add_attribute("push_unmixed_votes", "vote")
        .add_attribute("poll_id", poll_id.to_string()))
}

fn tally_votes(storage: &mut dyn Storage, poll_id: u64) -> Result<(u64, u64), ContractError> {
//     TODO
    let mut poll = match POLLS.may_load(storage, poll_id)? {
        Some(poll) => Some(poll),
        None => return Err(ContractError::PollNotExist {}),
    }.unwrap();

    let mut up_votes = 0u64;
    let mut down_votes = 0u64;

    poll.votes.list = poll.votes.list.iter().map(|item| {
        // TODO make less dirty
        let mut vote = item.to_owned();

        if vote.decrypted_vote_kind.is_none() {
            return vote
        }

        let strings: Vec<&str> = vote.encrypted_vote.clone().split(".").collect();
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
        vote
    }).collect();

    POLLS.save(storage, poll.id, &poll).unwrap();

    Ok((up_votes, down_votes))
}

fn is_admin(storage: &dyn Storage, addr: Addr) -> bool {
    CONFIG.load(storage).unwrap().admins.contains(&addr)
}

fn execute_close_poll(
    storage: &mut dyn Storage,
    poll_id: u64,
    kind: ClosePollKind,
    block: &BlockInfo,
    addr: Addr,
) -> Result<Response, ContractError> {
    if !is_admin(storage, addr) {
        return Err(ContractError::InvalidAuthorisation {});
    }

    // TODO: check If already passed/closed etc

    let mut poll = match POLLS.may_load(storage, poll_id)? {
        Some(poll) => Some(poll),
        None => return Err(ContractError::PollNotExist {}),
    }
    .unwrap();

    if poll.status != Active {
        return Err(ContractError::InactivePoll {});
    }

    poll.status = match kind {
        ClosePollKind::Passed => PollStatus::Passed,
        ClosePollKind::Rejected => PollStatus::Rejected,
    };

    (poll.votes.up_votes, poll.votes.down_votes) = tally_votes(
        storage,
        poll_id,
    ).unwrap();

    // update poll state after tallying
    poll.update_status(block, storage);

    POLLS.save(storage, poll.id, &poll).unwrap();

    Ok(Response::new()
        .add_attribute("action", "close")
        .add_attribute("status", kind.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Poll { poll_id } => query_poll(deps.storage, poll_id),
        QueryMsg::Polls { status } => query_polls(deps.storage, status, &env.block),
        QueryMsg::VoterInfo { addr } => {
            to_binary(&VOTERS.load(deps.storage, deps.api.addr_validate(&addr)?)?)
        }
        QueryMsg::IsAdmin { addr } => to_binary(&IsAdminResponse {
            is_admin: is_admin(deps.storage, deps.api.addr_validate(&addr)?),
        }),
        // Query number of votes in poll
        QueryMsg::PollVotes { poll_id } => query_poll_vote_count(deps.storage, poll_id),
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
        // public_key: poll.public_key,
    })
}

fn query_polls(storage: &dyn Storage, status: PollStatus, block: &BlockInfo) -> StdResult<Binary> {
    to_binary(&PollsResponse {
        polls: POLLS
            .range(storage, None, None, cosmwasm_std::Order::Ascending)
            .filter(|item| {
                let (_, mut poll) = item.as_ref().unwrap();
                let resolved_status = if poll.status == PollStatus::Active && poll.has_expired(block) { PollStatus::Pending } else { poll.status };
                resolved_status == status
            })
            .map(|item| {
                let (_, poll) = item.unwrap();
                PollResponse {
                    creator: poll.creator,
                    kind: poll.kind,
                    status: if poll.status == PollStatus::Active && poll.has_expired(block) { PollStatus::Pending } else { poll.status },
                    threshold_percentage: poll.threshold_percentage,
                    start_height: poll.start_height,
                    end_height: poll.end_height,
                    title: poll.title,
                    description: poll.description,
                    // public_key: poll.public_key,
                }
            })
            .collect::<Vec<PollResponse>>(),
    })
}

fn query_poll_vote_count(storage: &dyn Storage, poll_id: u64) -> StdResult<Binary> {
    to_binary(&PollVoteCountResponse {
        count: POLLS.load(storage, poll_id)?.votes,
    })
}
