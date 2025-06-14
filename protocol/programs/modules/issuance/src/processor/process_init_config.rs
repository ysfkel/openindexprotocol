use borsh::BorshDeserialize;
use openindex_sdk::openindex::state::Protocol;
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};




pub fn process_init_config(program_id: &Pubkey, accounts: &[AccountInfo], hooks: Vec<Pubkey>) -> ProgramResult {

    let account_info = &mut accounts.iter();
    let signer = next_account_info(account_info)?;
    let protocol_account = next_account_info(account_info)?;

    if !signer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

   let protocol: Protocol = Protocol::try_from_slice(&protocol_account.data.borrow())?;

   if signer.key != &protocol.owner {

   }






    Ok(())
}