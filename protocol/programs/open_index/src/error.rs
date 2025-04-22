use solana_program::{address_lookup_table::error, program_error::ProgramError};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ProtocolError {
    #[error("Error:Invalid token mint")]
    InvalidTokenMint,
    #[error("Error:Invalid mint account")]
    InvalidMintAccount,
    #[error("Error:Invalid max index components")]
    InvalidMaxIndexComponents,
    #[error("Error:Invalid token account")]
    InvalidTokenAccount,
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
    #[error("Error:Invalid protocol account owner")]
    UnknownProtocolAccount,
    #[error("Error:Invalid Module owner")]
    UnknownModuleAccount,
    #[error("Error:Invalid Index Account")]
    UnknownIndexAccount,
    #[error("Error:Mints amounts lengths (len) mismatch")]
    MintsAmountsLenMismatch,
    #[error("Error:Invalid mint")]
    InvalidMint,
    #[error("Error:Component amount error")]
    ComponentAmountError,
    #[error("Error:Index not initialized")]
    IndexNotInitialized,
}

impl From<ProtocolError> for ProgramError {
    fn from(e: ProtocolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

#[macro_export]
macro_rules! require {
    ($cond:expr, $err:expr, $msg:expr) => {
        if !$cond {
            msg!($msg);
            return Err($err);
        }
    };
}
