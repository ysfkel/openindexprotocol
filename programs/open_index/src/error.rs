use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ProtocolError {
    #[error("Error:Provided system program account is incorrect")]
    InvalidSystemProgramAccount,
    #[error("Error:Invalid token mint")]
    InvalidTokenMint,
    #[error("Error:Invalid mint account")]
    InvalidMintAccount,
    #[error("Error:Invalid max index components")]
    InvalidMaxIndexComponents,
    #[error("Error:Invalid associated token program account")]
    InvalidAssociatedTokenProgramAccount,
    #[error("Error:Invalid token program account")]
    InvalidTokenProgramAccount,
    #[error("Error:incorrect protocol account")]
    IncorrectProtocolAccount,
    #[error("Error:incorrect controller account")]
    IncorrectControllerAccount,
    #[error("Error:incorrect controller global config account")]
    IncorrectControllerGlobalConfigAccount,
    #[error("Error:incorrect vault account")]
    IncorrectVaultAccount,
    #[error("Error:Protocol not initialized")]
    ProtocolNotInitialized,
    #[error("Error:Incorrect index account")]
    IncorrectIndexAccount,
    #[error("Error:Incorrect component account")]
    IncorrectComponentAccount,
    #[error("Error:Incorrect vault ata")]
    IncorrectVaultATA,
    #[error("Error:Controller glocal confog not initialized")]
    ControllerGlobalConfigNotInitialized,
    #[error("Error:Only protocol owner can execute this instruction")]
    OnlyProtocolOwnerCanExecuteThisInstruction,
    #[error("Error:Only controller owner can execute this instruction")]
    OnlyControllerOwnerCanExecuteThisInstruction,
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
    #[error("Error:Mints amounts lengths (len) mismatch")]
    MintsAmountsLenMismatch,
    #[error("Error:Invalid mint")]
    InvalidMint,
    #[error("Error:Component amount error")]
    ComponentAmountError,
}

impl From<ProtocolError> for ProgramError {
    fn from(e: ProtocolError) -> Self {
        ProgramError::Custom(e as u32)
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
