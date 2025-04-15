use solana_program::{address_lookup_table::error, program_error::ProgramError};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ProtocolError {
    #[error("Error:Incorrect module account")]
    IncorrectModuleAccount,
    #[error("Error:Unknown module account")]
    UnknownModuleAccount,
    #[error("Error:Invalid registered module account")]
    InvalidRegisredModuleAccount,
    #[error("Error:Invalid open index account")]
    InvalidOpenIndexAccount,
    #[error("Error:Invalid vault_ata")]
    IncorrectVaultATA,
    #[error("Error: invalid open index program account")]
    InvalidMintAccount,
    #[error("Error:Incorrect index mints account")]
    IncorrectIndexMintsAccount,
    #[error("Error:Incorrect vault account")]
    IncorrectVaultAccount,
    #[error("Error:Incorrect component account")]
    IncorrectComponentAccount,
    #[error("Error: Amount must be greater than zero")]
    AmountMustBeGreaterThanZero,
    #[error("Error:Component not initialized")]
    ComponentNotInitialized,
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
