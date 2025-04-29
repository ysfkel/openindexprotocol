use crate::processor;
use solana_program::{
    account_info::AccountInfo,
    address_lookup_table::{instruction, program},
    entrypoint::{entrypoint, ProgramResult},
    msg,
    pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction: instruction received program_id {:?}",
        program_id
    );
    processor::process_instruction(program_id, accounts, instruction_data)?;
    Ok(())
}
