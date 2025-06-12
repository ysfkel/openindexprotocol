use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::processor;

 pub fn entry_point(program_id: &Pubkey, 
  accounts: &[AccountInfo],
  instruction_data: &[u8]
) -> ProgramResult {

    if let Err(err) = processor::process_instruction(program_id,accounts , instruction_data) {
        
    }

    Ok(())
}