//! Program state processor

use crate::state::Protocol;
use borsh::BorshSerialize;
use openindex_sdk::{
    openindex::{error::ProtocolError, pda::find_protocol_address, seeds::PROTOCOL_SEED},
    require,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

//// instruction to process initializing a protocol
pub fn process_init_protocol(program_id: Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let signer = next_account_info(accounts_iter)?;
    let protocol_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;

    require!(signer.is_signer, ProgramError::MissingRequiredSignature);

    require!(
        protocol_account.lamports() == 0,
        ProgramError::AccountAlreadyInitialized
    );

    let (protocol_pda, protocol_bump) = find_protocol_address(&program_id);

    require!(
        *protocol_account.key == protocol_pda,
        ProtocolError::IncorrectProtocolAccount.into()
    );

    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(Protocol::LEN);

    invoke_signed(
        &system_instruction::create_account(
            &signer.key,
            &protocol_account.key,
            lamports,
            Protocol::LEN as u64,
            &program_id,
        ),
        &[
            signer.clone(),
            protocol_account.clone(),
            system_program_account.clone(),
        ],
        &[&[PROTOCOL_SEED, &[protocol_bump]]],
    )?;

    let protocol = Protocol::new(signer.key.clone(), protocol_bump);
    protocol.serialize(&mut &mut protocol_account.data.borrow_mut()[..])?;

    Ok(())
}
