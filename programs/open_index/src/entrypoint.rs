use solana_program::{
    account_info::AccountInfo,
    address_lookup_table::{instruction, program},
    entrypoint::{entrypoint, ProgramResult},
    msg,
    pubkey::Pubkey,
};

// token vaults
// vault_pda("vault", index_pubkey, mint_pubkey)

use crate::processor;

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
