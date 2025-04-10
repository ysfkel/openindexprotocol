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

use crate::{
    error::ProtocolError,
    seeds::{CONTROLLER_GLOBAL_CONFIG_SEED, PROTOCOL_SEED},
    state::{ControllerGlobalConfig, Protocol},
};

pub fn init_controller_global_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    max_index_components: u32,
) -> ProgramResult {
    msg!("initializing controlelr global config");

    if max_index_components == 0 {
        return Err(ProtocolError::InvalidMaxIndexComponents.into());
    }

    let accounts_iter = &mut accounts.iter();
    let owner = next_account_info(accounts_iter)?;
    let protocol_account = next_account_info(accounts_iter)?;
    let controller_global_config_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;

    if controller_global_config_account.lamports() > 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut protocol: Protocol = Protocol::try_from_slice(&protocol_account.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?;

    if !protocol.is_initialized() {
        return Err(ProtocolError::ProtocolNotInitialized.into());
    }

    let (protocol_pda, bump) = Pubkey::find_program_address(&[&PROTOCOL_SEED], &program_id);

    if *protocol_account.key != protocol_pda {
        return Err(ProtocolError::IncorrectProtocolAccount.into());
    }

    if *owner.key != protocol.owner {
        return Err(ProtocolError::OnlyProtocolOwnerCanExecuteThisInstruction.into());
    }

    let (controller_global_config_pda, controller_global_conifg_bump) =
        Pubkey::find_program_address(&[CONTROLLER_GLOBAL_CONFIG_SEED], &program_id);

    if *controller_global_config_account.key != controller_global_config_pda {
        return Err(ProtocolError::IncorrectControllerGlobalConfigAccount.into());
    }

    msg!(
        "controller global config bump: {}",
        controller_global_conifg_bump
    );
    msg!("max index components: {}", max_index_components);

    if *system_program_account.key != system_program::id() {
        return Err(ProtocolError::InvalidSystemProgramAccount.into());
    }

    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(ControllerGlobalConfig::LEN);

    invoke_signed(
        &system_instruction::create_account(
            &owner.key,
            &controller_global_config_account.key,
            lamports,
            ControllerGlobalConfig::LEN as u64,
            &program_id,
        ),
        &[
            owner.clone(),
            controller_global_config_account.clone(),
            system_program_account.clone(),
        ],
        &[&[
            CONTROLLER_GLOBAL_CONFIG_SEED,
            &[controller_global_conifg_bump],
        ]],
    )?;

    let mut controller_global_conifg =
        ControllerGlobalConfig::new(max_index_components, controller_global_conifg_bump);
    controller_global_conifg
        .serialize(&mut &mut controller_global_config_account.data.borrow_mut()[..])?;

    msg!(
        "controller global config initialized {:?}",
        controller_global_config_account.key
    );
    Ok(())
}
