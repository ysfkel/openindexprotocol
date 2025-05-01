use crate::processor::{
    process_add_index_components, process_create_index, process_init_controller, process_init_controller_global_config,
    process_init_module, process_init_protocol, process_mint, process_redeem,
};
use borsh::BorshDeserialize;
use openindex_sdk::openindex::instruction::ProtocolInstruction as Instruction;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(instruction_data)?;

    match instruction {
        Instruction::InitProtocol => process_init_protocol(program_id.clone(), accounts)?,

        Instruction::InitController => process_init_controller(program_id, accounts)?,

        Instruction::InitControllerGlobalConfig {
            max_index_components,
        } => process_init_controller_global_config(program_id, accounts, max_index_components)?,

        Instruction::CreateIndex => process_create_index(program_id, accounts)?,
        Instruction::AddIndexComponents { amounts, mints } => {
            process_add_index_components(program_id, accounts, mints, amounts)?
        }
        Instruction::Mint { index_id, amount } => process_mint(program_id, accounts, index_id, amount)?,
        Instruction::Redeem { index_id, amount } => process_redeem(program_id, accounts, index_id, amount)?,
        Instruction::InitModule => process_init_module(program_id, accounts)?, 
        _ => Err(ProgramError::InvalidInstructionData)?,
    }

    Ok(())
}
