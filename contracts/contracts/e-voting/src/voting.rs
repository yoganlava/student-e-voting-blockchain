use crate::error::ContractError;
use crate::msg::{Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::response::{IsAdminResponse, PollResponse, PollsResponse};

use crate::state::{next_poll_id, Config, Poll, PollStatus, PollVote, PollVotes, Voter, CONFIG, POLLS, VOTERS, OpaquePollVotes};
use cosmwasm_std::{entry_point, to_binary, Addr, Binary, BlockInfo, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult, Storage, CosmosMsg, WasmMsg, Uint128, from_binary};
use cw20::Cw20ReceiveMsg;


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
        ExecuteMsg::Receive(msg) => handle_receive_message(deps.storage, msg, info, env),
        ExecuteMsg::CastVote {
            poll_id,
            encrypted_vote,
        } => execute_cast_vote(
                deps.storage,
                PollVote {
                    voter_addr: info.sender,
                    poll_id,
                    decrypted_vote: "".to_owned(),
                    decrypted_vote_kind: None,
                    encrypted_vote,
                    decrypted_vote_tracker: None,
                    malformed: false
                },
                &env.block,
            ),
        // TODO MAKE AND THEN START FRONTEND
        ExecuteMsg::RegisterVoter {
            name,
            email,
            student_code,
        } => execute_register_voter(deps.storage, Voter {
                name,
                email,
                student_code,
                addr: info.sender
            }),
        // Veto close
        ExecuteMsg::ChangeConfig { config } => execute_change_config(deps.storage, config, info.sender),
        ExecuteMsg::PushUnmixedVotes { poll_id, votes } => execute_push_unmixed_votes(deps.storage, poll_id, votes, info.sender, &env.block),
        // Prematurely close poll for tallying before end height is reached
        ExecuteMsg::ClosePoll { poll_id } => execute_close_poll(deps.storage, poll_id, info.sender)
        // TODO: Gift Execute message
    }
}

fn handle_receive_message(storage: &mut dyn Storage, msg: Cw20ReceiveMsg, info: MessageInfo, env: Env) -> Result<Response, ContractError> {
    return match from_binary::<Cw20HookMsg>(&msg.msg) {
        Ok(Cw20HookMsg::CreatePoll {
               title,
               kind,
               description,
               start_time,
               end_time,
           }) => {
            // TODO: dont hardcode
            if msg.amount < Uint128::from(10000u32) {
                return Err(ContractError::InvalidAmountPaid(Uint128::from(10000u32), msg.amount))
            }
            create_poll(
                storage,
                Poll {
                    id: 0,
                    creator: info.sender,
                    kind,
                    status: PollStatus::Active,
                    threshold_percentage: None,
                    start_time: start_time.unwrap_or(env.block.time),
                    end_time,
                    title,
                    description,
                    votes: PollVotes {
                        total: 0,
                        up_votes: 0,
                        down_votes: 0,
                        malformed_votes: 0,
                        list: vec![]
                    }
                }
            )
        },
        _ => Err(ContractError::Std(StdError::generic_err("Bad Message"))),
    }
}

fn execute_close_poll(
    storage: &mut dyn Storage,
    poll_id: u64,
    sender: Addr
) -> Result<Response, ContractError> {
    let mut poll = match POLLS.may_load(storage, poll_id)? {
        Some(poll) => Some(poll),
        None => return Err(ContractError::PollNotExist {}),
    }.unwrap();

    if poll.creator != sender {
        return Err(ContractError::InvalidAuthorisation {});
    }

    if poll.status != PollStatus::Active {
        return Err(ContractError::PollNotActive {})
    }

    poll.status = PollStatus::Pending;

    POLLS.save(storage, poll_id, &poll);

    Ok(Response::new()
        .add_attribute("action", "close_poll")
        .add_attribute("poll_id", poll_id.to_string())
    )
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
    sender: Addr,
) -> Result<Response, ContractError> {
    if !is_admin(storage, sender) {
        return Err(ContractError::InvalidAuthorisation {});
    }

    CONFIG.save(storage, &config).unwrap();

    Ok(Response::new().add_attribute("action", "change_config"))
}

fn create_poll(
    storage: &mut dyn Storage,
    mut poll: Poll
) -> Result<Response, ContractError> {
    // TODO verification
    // TODO ask for token fee of like 10 SVT idk

    let poll_id = next_poll_id(storage)?;
    poll.id = poll_id;
    POLLS.save(storage, poll.id, &poll)?;

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
        decrypted_vote: "".to_owned(),
        encrypted_vote: vote.encrypted_vote,
        decrypted_vote_tracker: None,
        malformed: false
    });

    POLLS.save(storage, poll.id, &poll).unwrap();

    Ok(Response::new()
        .add_attribute("action", "vote"))
}

fn execute_push_unmixed_votes(storage: &mut dyn Storage, poll_id: u64, votes: Vec<PollVote>, sender: Addr, block: &BlockInfo) -> Result<Response, ContractError> {
    if !is_mixnet(storage, sender) {
        return Err(ContractError::InvalidAuthorisation {})
    }

    let mut poll = match POLLS.may_load(storage, poll_id)? {
        Some(poll) => Some(poll),
        None => return Err(ContractError::PollNotExist {}),
    }.unwrap();

    poll.votes.list = votes;

    poll.tally_votes();

    // update poll state after tallying
    poll.update_status(block, storage);

    POLLS.save(storage, poll.id, &poll).unwrap();

    Ok(Response::new()
        .add_attribute("push_unmixed_votes", "vote")
        .add_attribute("poll_id", poll_id.to_string()))
}

fn is_admin(storage: &dyn Storage, addr: Addr) -> bool {
    CONFIG.load(storage).unwrap().admins.contains(&addr)
}

fn is_mixnet(storage: &dyn Storage, addr: Addr) -> bool {
    CONFIG.load(storage).unwrap().mixnet_addr == addr
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Poll { poll_id } => query_poll(deps.storage, poll_id),
        // TODO: Polls { limit: u64 } => .take(limit) => { active_polls: [],  pending_polls: [], passed_polls: [], rejected_polls: [] }
        QueryMsg::Polls { status } => query_polls(deps.storage, status, &env.block),
        QueryMsg::VoterInfo { addr } => {
            to_binary(&VOTERS.load(deps.storage, deps.api.addr_validate(&addr)?)?)
        }
        QueryMsg::IsAdmin { addr } => to_binary(&IsAdminResponse {
            is_admin: is_admin(deps.storage, deps.api.addr_validate(&addr)?),
        }),
        QueryMsg::PollVotes { poll_id } => query_poll_votes(deps.storage, poll_id),
        QueryMsg::ParticipatedPolls {
            addr
        } => query_participated_poll(deps.storage, deps.api.addr_validate(&addr)?, &env.block),
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage).unwrap()),
    //     TODO a query to check if a voter has voted in the poll
    //     TODO: allow to view vote details after passed/rejected

    //     TODO: Query gift notifications

    }
}

fn query_participated_poll(storage: &dyn Storage, voter_addr: Addr, block: &BlockInfo) -> StdResult<Binary> {
    return to_binary(&POLLS
        .range(storage, None, None, cosmwasm_std::Order::Ascending)
        .filter(|item| {
            let (_, poll) = item.clone().as_ref().unwrap();
            poll.votes.list.iter().any(|vote| vote.voter_addr == voter_addr)
        }).map(|item| {
        let (_, poll) = item.unwrap();
        PollResponse {
            id: poll.id,
            creator: poll.creator.clone(),
            kind: poll.kind.clone(),
            status: if poll.status == PollStatus::Active && poll.has_expired(block) { PollStatus::Pending } else { poll.status },
            threshold_percentage: poll.threshold_percentage,
            start_time: poll.start_time,
            end_time: poll.end_time,
            title: poll.title,
            description: poll.description,
            votes: OpaquePollVotes {
                total: poll.votes.total,
                up_votes: poll.votes.up_votes,
                down_votes: poll.votes.down_votes,
                malformed_votes: poll.votes.malformed_votes
            }
        }
    })
        .collect::<Vec<PollResponse>>())
}

fn query_poll(storage: &dyn Storage, poll_id: u64) -> StdResult<Binary> {
    let poll = match POLLS.may_load(storage, poll_id)? {
        Some(poll) => Some(poll),
        None => return Err(StdError::generic_err("Poll does not exist")),
    }
    .unwrap();

    to_binary(&PollResponse {
        id: poll.id,
        creator: poll.creator,
        kind: poll.kind,
        status: poll.status,
        threshold_percentage: poll.threshold_percentage,
        start_time: poll.start_time,
        end_time: poll.end_time,
        title: poll.title,
        description: poll.description,
        votes: OpaquePollVotes {
            total: poll.votes.total,
            up_votes: poll.votes.up_votes,
            down_votes: poll.votes.down_votes,
            malformed_votes: poll.votes.malformed_votes
        }
    })
}

fn query_polls(storage: &dyn Storage, status: PollStatus, block: &BlockInfo) -> StdResult<Binary> {
    to_binary(&PollsResponse {
        polls: POLLS
            .range(storage, None, None, cosmwasm_std::Order::Ascending)
            .filter(|item| {
                let (_, poll) = item.clone().as_ref().unwrap();
                let resolved_status = if poll.status == PollStatus::Active && poll.has_expired(block) { PollStatus::Pending } else { poll.status.clone() };
                resolved_status == status
            })
            .map(|item| {
                let (_, poll) = item.unwrap();
                PollResponse {
                    id: poll.id,
                    creator: poll.creator.clone(),
                    kind: poll.kind.clone(),
                    status: if poll.status == PollStatus::Active && poll.has_expired(block) { PollStatus::Pending } else { poll.status },
                    threshold_percentage: poll.threshold_percentage,
                    start_time: poll.start_time,
                    end_time: poll.end_time,
                    title: poll.title,
                    description: poll.description,
                    votes: OpaquePollVotes {
                        total: poll.votes.total,
                        up_votes: poll.votes.up_votes,
                        down_votes: poll.votes.down_votes,
                        malformed_votes: poll.votes.malformed_votes
                    }
                }
            })
            .collect::<Vec<PollResponse>>(),
    })
}

fn query_poll_votes(storage: &dyn Storage, poll_id: u64) -> StdResult<Binary> {
    // ? Maybe only allow queries after poll ends?
    to_binary(&POLLS.load(storage, poll_id)?.votes.list)
}

fn query_participated_polls(_storage: &dyn Storage, _addr: Addr) -> StdResult<Binary> {
    todo!()
}