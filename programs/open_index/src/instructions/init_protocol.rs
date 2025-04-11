use crate::{error::ProtocolError, seeds::PROTOCOL_SEED, state::Protocol};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};

pub fn init_protocol(program_id: Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let owner = next_account_info(accounts_iter)?;
    let protocol_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;

    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if protocol_account.lamports() > 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if *system_program_account.key != system_program::id() {
        return Err(ProtocolError::InvalidSystemProgramAccount.into());
    }

    let (protocol_pda, protocol_bump) = Pubkey::find_program_address(&[PROTOCOL_SEED], &program_id);

    if *protocol_account.key != protocol_pda {
        return Err(ProtocolError::IncorrectProtocolAccount.into());
    }

    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(Protocol::LEN);

    msg!("Creating protocol account with bump {}", protocol_bump);

    invoke_signed(
        &system_instruction::create_account(
            &owner.key,
            &protocol_account.key,
            lamports,
            Protocol::LEN as u64,
            &program_id,
        ),
        &[
            owner.clone(),
            protocol_account.clone(),
            system_program_account.clone(),
        ],
        &[&[PROTOCOL_SEED, &[protocol_bump]]],
    )?;

    let protocol = Protocol::new(owner.key.clone(), protocol_bump);
    protocol.serialize(&mut &mut protocol_account.data.borrow_mut()[..])?;
    msg!("Protocol account initialized {:?}", protocol_account.key);

    Ok(())
}
