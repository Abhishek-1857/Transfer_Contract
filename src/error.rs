use cosmwasm_std::{StdError, Addr};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Insufficient funds (balance {balance}, required={required})")]
    InsufficientFunds { balance: u128, required: u128 },

    #[error("Insufficient freeze funds (balance {balance}, required={required})")]
    InsufficientFreezeFunds { balance: u128, required: u128 },

    #[error("Only freeze fund is left (balance {balance})")]
    OnlyFeezeFundLeft { balance: u128},

    #[error("Country Code (country_code {country_code}) is of UnauthorisedCountry")]
    UnauthorisedCountry { country_code: u128},

    #[error("Corrupted data found (16 byte expected)")]
    CorruptedDataFound {},

    #[error("Account is already Freezed")]
    CanNotFreezeAccount {},

    #[error("Account is Freezed , can not transfer or trade")]
    FreezedAccount {},

    #[error("You are not Admin, can't perform this action")]
    NotAdmin {},

    #[error("Cannot Remove from shareholders as (address {address}) has (balance {balance}) left")]
    CanNotRemove {address : Addr,balance : u128},


    #[error("(address {address}) has reached maximum Holder balance , can't send more ")]
    MaxHolderBalance {address : Addr},

    #[error("proposal_id {proposal_id} is invalid ! ")]
    InvalidProposalId{proposal_id : u128},

    #[error("Address {address} is not a signer who can approve")]
    Unauthorised{address : Addr},

    #[error("Address {address} has already approved")]
    AlreadyApproved{address : Addr},

    #[error("Need more than half of signers approval. Current approvals are {approvals}")]
    NotEnoughApproval{approvals : u128},

    #[error("You are not owner of mint proposal, can't perform this action")]
    NotOwner {},

    #[error("The proposal with proposal_id {proposal_id} has been complete")]
    Completed {proposal_id: u128}
}