use crate::instructions::mint_index;
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};
use openindex_sdk::issuance::instruction::IssuanceInstruction;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = IssuanceInstruction::try_from_slice(instruction_data)?;

    match instruction {
        IssuanceInstruction::Mint { index_id, amount } => {
            mint_index(program_id, accounts, index_id, amount)?
        }

        _ => Err(ProgramError::InvalidInstructionData)?,
    }

    Ok(())
}
