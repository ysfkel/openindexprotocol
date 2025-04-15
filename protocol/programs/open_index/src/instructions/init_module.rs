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
    system_instruction, system_program as sys_program,
    sysvar::Sysvar,
};

use crate::{
    error::ProtocolError,
    state::{Module, Protocol},
};
use open_index_lib::seeds::{MODULE_SEED, PROTOCOL_SEED};
pub fn init_module(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let owner = next_account_info(accounts_iter)?;
    let protocol_account = next_account_info(accounts_iter)?;
    let module_account = next_account_info(accounts_iter)?;
    let registered_module_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if protocol_account.owner != program_id {
        return Err(ProtocolError::UnknownProtocolAccount.into());
    }

    let mut protocol: Protocol = Protocol::try_from_slice(&protocol_account.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?;

    if protocol.owner != *owner.key {
        return Err(ProtocolError::OnlyProtocolOwner.into());
    }

    if module_account.lamports() > 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if !protocol.is_initialized() {
        return Err(ProtocolError::ProtocolNotInitialized.into());
    }

    let protocol_pda =
        Pubkey::create_program_address(&[&PROTOCOL_SEED, &[protocol.bump]], program_id)?;

    if *protocol_account.key != protocol_pda {
        return Err(ProtocolError::IncorrectProtocolAccount.into());
    }

    //// REGISTER MODULE

    let (registered_module_pda, registered_module_bump) =
        Pubkey::find_program_address(&[&MODULE_SEED, &module_account.key.as_ref()], program_id);

    msg!("MODULE:: Module pda {:?}", module_account.key);

    msg!(
        "registered_module_bump ID: {}, bump: {}",
        registered_module_pda,
        registered_module_bump
    );

    if *registered_module_account.key != registered_module_pda {
        return Err(ProtocolError::IncorrectModuleAccount.into());
    }

    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(Module::LEN);

    msg!(
        "invoking system_instruction::create_account -> module {:?}",
        registered_module_account.key
    );
    invoke_signed(
        &system_instruction::create_account(
            &owner.key,
            &registered_module_account.key,
            lamports,
            Module::LEN as u64,
            program_id,
        ),
        &[
            owner.clone(),
            registered_module_account.clone(),
            system_program.clone(),
        ],
        &[&[
            MODULE_SEED,
            &module_account.key.as_ref(),
            &[registered_module_bump],
        ]],
    )?;

    // initialize module
    let mut module = Module::new(
        registered_module_account.key.clone(),
        true,
        registered_module_bump,
    );
    module.serialize(&mut &mut registered_module_account.data.borrow_mut()[..])?;

    msg!("module initialized {:?}", registered_module_account.key);
    Ok(())
}
