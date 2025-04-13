use crate::{
    instruction::Instruction,
    instructions::{
        create_index, init_controller, init_controller_global_config, init_module, init_protocol,
        mint_index,
    },
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
        Instruction::InitProtocol => init_protocol(program_id.clone(), accounts)?,

        Instruction::InitController => init_controller(program_id, accounts)?,

        Instruction::InitControllerGlobalConfig {
            max_index_components,
        } => init_controller_global_config(program_id, accounts, max_index_components)?,

        Instruction::CreateIndex { amounts, mints } => create_index(program_id, accounts)?,
        Instruction::MintIndex {
            index_id,
            amount,
            to,
        } => mint_index(program_id, accounts, index_id, amount)?,
        Instruction::InitModule => init_module(program_id, accounts)?,
        Instruction::InitController => init_controller(program_id, accounts)?,
        _ => Err(ProgramError::InvalidInstructionData)?,
    }

    Ok(())
}
