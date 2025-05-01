use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program as sys_program,
    sysvar::Sysvar,
};

use crate::state::{Module, Protocol};
use openindex_sdk::{
    openindex::{
        error::ProtocolError,
        seeds::{MODULE_SEED, PROTOCOL_SEED},
    },
    require,
};
pub fn init_module(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let owner = next_account_info(accounts_iter)?;
    let protocol_account = next_account_info(accounts_iter)?;
    let module_signer_account = next_account_info(accounts_iter)?;
    let registered_module_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    require!(owner.is_signer, ProgramError::MissingRequiredSignature);

    require!(
        protocol_account.owner == program_id,
        ProtocolError::UnknownProtocolAccount.into()
    );

    let protocol: Protocol = Protocol::try_from_slice(&protocol_account.data.borrow())
        .map_err(|_| ProtocolError::InvalidProtocolAccountData)?;

    require!(
        protocol.owner == *owner.key,
        ProtocolError::OnlyProtocolOwner.into()
    );

    require!(
        protocol.is_initialized(),
        ProtocolError::ProtocolNotInitialized.into()
    );

    let protocol_pda =
        Pubkey::create_program_address(&[&PROTOCOL_SEED, &[protocol.bump]], program_id)?;

    require!(
        *protocol_account.key == protocol_pda,
        ProtocolError::IncorrectProtocolAccount.into()
    );

    let (registered_module_pda, registered_module_bump) = Pubkey::find_program_address(
        &[&MODULE_SEED, &module_signer_account.key.as_ref()],
        program_id,
    );

    require!(
        *registered_module_account.key == registered_module_pda,
        ProtocolError::IncorrectModuleAccount.into()
    );

    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(Module::LEN);

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
            &module_signer_account.key.as_ref(),
            &[registered_module_bump],
        ]],
    )?;

    let mut module = Module::new(true, registered_module_bump);
    module.serialize(&mut &mut registered_module_account.data.borrow_mut()[..])?;

    Ok(())
}
