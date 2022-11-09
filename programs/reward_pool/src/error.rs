use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the Contract reward_pool.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum ContractError {
    #[error("An instruction's data contents was invalid")]
    InvalidInstructionData,
    #[error("Only admin can call this instruction")]
    UnauthorisedAccess,
    #[error("Can`t deserialize data")]
    DeserializeError,
    #[error("Address is not verified")]
    UnverifiedAddress,
    #[error("Nft from wrong collection")]
    WrongCreatorAddress,
}

impl From<ContractError> for ProgramError {
    fn from(e: ContractError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for ContractError {
    fn type_of() -> &'static str {
        "ContractError"
    }
}
