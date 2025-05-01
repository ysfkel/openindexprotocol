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

use crate::state::{Controller, Protocol};
use openindex_sdk::{
    openindex::{
        error::ProtocolError,
        seeds::{CONTROLLER_SEED, PROTOCOL_SEED},
    },
    require,
};
pub fn process_init_controller(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let owner = next_account_info(accounts_iter)?;
    let protocol_account = next_account_info(accounts_iter)?;
    let controller_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    require!(owner.is_signer, ProgramError::MissingRequiredSignature);

    require!(
        protocol_account.owner == program_id,
        ProtocolError::UnknownProtocolAccount.into()
    );

    require!(
        controller_account.lamports() == 0,
        ProgramError::AccountAlreadyInitialized
    );

    let mut protocol: Protocol = Protocol::try_from_slice(&protocol_account.data.borrow())
        .map_err(|_| ProtocolError::InvalidProtocolAccountData)?;

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

    let controller_id = protocol.get_next_controller_id();

    let (controller_pda, controller_bump) = Pubkey::find_program_address(
        &[&CONTROLLER_SEED, &controller_id.to_le_bytes()],
        program_id,
    );

    require!(
        *controller_account.key == controller_pda,
        ProtocolError::IncorrectControllerAccount.into()
    );

    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(Controller::LEN);

    invoke_signed(
        &system_instruction::create_account(
            &owner.key,
            &controller_account.key,
            lamports,
            Controller::LEN as u64,
            program_id,
        ),
        &[
            owner.clone(),
            controller_account.clone(),
            system_program.clone(),
        ],
        &[&[
            CONTROLLER_SEED,
            &controller_id.to_le_bytes(),
            &[controller_bump],
        ]],
    )?;

    let controller = Controller::new(controller_id, owner.key.clone(), controller_bump);
    controller.serialize(&mut &mut controller_account.data.borrow_mut()[..])?;

    protocol.generate_next_controller_id();
    protocol.serialize(&mut &mut protocol_account.data.borrow_mut()[..])?;

    Ok(())
}
