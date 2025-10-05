use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Reward not found")]
    RewardNotFound {},

    #[error("Rule not found")]
    RuleNotFound {},

    #[error("Contract not found")]
    ContractNotFound {},

    #[error("User not found")]
    UserNotFound {},

    #[error("Invalid amount")]
    InvalidAmount {},

    #[error("Reward already claimed")]
    RewardAlreadyClaimed {},

    #[error("Reward expired")]
    RewardExpired {},

    #[error("Rule already exists")]
    RuleAlreadyExists {},

    #[error("Contract already registered")]
    ContractAlreadyRegistered {},

    #[error("Invalid configuration")]
    InvalidConfiguration {},

    #[error("Insufficient balance")]
    InsufficientBalance {},

    #[error("Operation not allowed")]
    OperationNotAllowed {},

    #[error("System error: {msg}")]
    SystemError { msg: String },
}
