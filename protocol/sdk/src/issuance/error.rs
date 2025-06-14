use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError,program_error::PrintProgramError, msg, program_error::ProgramError};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum IssuanceError {
#[error("Error:Incorrect index issuance config account")]
 IncorrectIssuanceConfigAccount,
 #[error("Error:Incorrecthook account")]
 IncorrectHookAccount,

#[error("Error:Invalid issuance config account owner")]
 UnknownIssuanceConfigAccount,
}

impl From<IssuanceError> for ProgramError {
    fn from(e: IssuanceError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl PrintProgramError for IssuanceError {
    fn print<E>(&self) {
        msg!("ISSUANCE-ERROR: {}", &self.to_string());
    }
}

impl<T> DecodeError<T> for IssuanceError {
    fn type_of() -> &'static str {
        "Issuance Error"
    }
}
