use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum ProtocolError {
    #[error("Error:Invalid token account")]
    InvalidTokenAccount = 500,
    #[error("Error:Invalid protocol account data")]
    InvalidProtocolAccountData,
    #[error("Error:Amount must be greater than zero")]
    AmountMustBeGreaterThanZero,
    #[error("Error:Invalid token mint")]
    InvalidTokenMint,
    #[error("Error:Invalid mint account")]
    InvalidMintAccount,
    #[error("Error:Invalid max index components")]
    InvalidMaxIndexComponents,
    #[error("Invalid index mints account data")]
    InvalidIndexMintsAccountData,
    #[error("Invalid component account data")]
    InvalidComponentData,

    #[error("Error:incorrect protocol account")]
    IncorrectProtocolAccount,
    #[error("Error:incorrect mint authority")]
    IncorrectMintAuthority,
    #[error("Error:Invalid module account")]
    InvalidRegisredModuleAccount,
    #[error("Error:incorrect controller account")]
    IncorrectControllerAccount,
    #[error("Error:incorrect controller global config account")]
    IncorrectControllerGlobalConfigAccount,
    #[error("Error:incorrect vault account")]
    IncorrectVaultAccount,
    #[error("Error:Incorrect mint account")]
    IncorrectMintAccount,
    #[error("Error:Protocol not initialized")]
    ProtocolNotInitialized,
    #[error("Error:Incorrect index account")]
    IncorrectIndexAccount,
    #[error("Error:Incorrect index mints account")]
    IncorrectIndexMintsAccount,
    #[error("Error:Incorrect module account")]
    IncorrectModuleAccount,
    #[error("Error:Incorrect component account")]
    IncorrectComponentAccount,
    #[error("Error:Incorrect vault ata")]
    IncorrectVaultATA,
    #[error("Error:Controller glocal confog not initialized")]
    ControllerGlobalConfigNotInitialized,
    #[error("Error:Only protocol owner can execute this instruction")]
    OnlyProtocolOwner,
    #[error("Error:Only controller owner can execute this instruction")]
    OnlyControllerOwner,
    #[error("Error:Only active modules can execute this instruction")]
    OnlyActiveModules,
    #[error("Error:Max index components exceeded")]
    MaxIndexComponentsExceeded,
    #[error("Error:No mints provided")]
    NoMintsProvided,
    #[error("Error:Invalid controller global config account owner")]
    UnknownControllerGlobalConfigAccount,
    #[error("Error:Invalid controller account owner")]
    UnknownControllerAccount,
    #[error("Error:Invalid index account")]
    UnknownIndexAccount,
    #[error("Error: Invalid index mints account")]
    UnknownIndexMintsAccount,
    #[error("Error:Invalid protocol account owner")]
    UnknownProtocolAccount,
    #[error("Error:Invalid Module owner")]
    UnknownModuleAccount,
    #[error("Error:Mints amounts lengths (len) mismatch")]
    MintsAmountsLenMismatch,
    #[error("Error:Invalid mint")]
    InvalidMint,
    #[error("Error:Component amount error")]
    ComponentAmountError,
    #[error("Error: Comoponent not initialized")]
    ComponentNotInitialized,
    #[error("Error:Index not initialized")]
    IndexNotInitialized,
}

impl From<ProtocolError> for ProgramError {
    fn from(e: ProtocolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl PrintProgramError for ProtocolError {
    fn print<E>(&self) {
        msg!("PROTOCOL-ERROR: {}", &self.to_string());
    }
}

impl<T> DecodeError<T> for ProtocolError {
    fn type_of() -> &'static str {
        "Protocol Error"
    }
}

#[macro_export]
macro_rules! require {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

/// Instruction errors
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum TransactionBuilderError {
    #[error("Error: Creating mint_to instruction failed")]
    MintTo(ProgramError),
    #[error("Error: Number of accounts exceeds transaction accounts limit")]
    TransactionAccountsLimit,
}

impl From<ProgramError> for TransactionBuilderError {
    fn from(e: ProgramError) -> Self {
        TransactionBuilderError::MintTo(e)
    }
}
