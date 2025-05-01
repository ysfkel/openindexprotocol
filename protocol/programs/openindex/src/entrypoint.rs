use crate::processor;
use openindex_sdk::openindex::error::ProtocolError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::PrintProgramError,
    pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint::entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(err) = processor::process_instruction(program_id, accounts, instruction_data) {
        err.print::<ProtocolError>();
        return Err(err);
    }
    Ok(())
}
