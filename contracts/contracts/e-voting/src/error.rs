use cosmwasm_std::{OverflowError, StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("Poll does not exist")]
    PollNotExist {},

    #[error("Poll not active")]
    PollNotActive {},

    #[error("Voter does not exist")]
    VoterNotExist {},

    #[error("Voter already exists")]
    VoterAlreadyExist {},

    #[error("Already Voted")]
    AlreadyVoted {},

    #[error("Invalid Authorisation")]
    InvalidAuthorisation {},

    #[error("Invalid Amount Paid. Expected: {0} Got: {1}")]
    InvalidAmountPaid(Uint128, Uint128)
}
