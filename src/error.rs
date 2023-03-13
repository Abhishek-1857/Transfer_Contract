use cosmwasm_std::{StdError, Addr};
use thiserror::Error;

#[derive(Error, Debug)]
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
}