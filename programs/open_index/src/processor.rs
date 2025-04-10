use crate::{
    instruction::Instruction,
    instructions::{create_index, init_controller, init_protocol},
};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(instruction_data)?;

    match instruction {
        Instruction::InitProtocol {
            max_components_per_index,
        } => {
            msg!("InitProtocol instruction received");
            init_protocol(program_id.clone(), accounts, max_components_per_index)?
        }
        Instruction::CreateIndex { amounts, mints } => {
            create_index(program_id, accounts, mints, amounts)?
        }
        Instruction::InitController => init_controller(program_id, accounts)?,
        _ => Err(ProgramError::InvalidInstructionData)?,
    }

    Ok(())
}
