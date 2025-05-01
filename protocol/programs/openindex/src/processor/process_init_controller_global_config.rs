use crate::state::{ControllerGlobalConfig, Protocol};
use borsh::{BorshDeserialize, BorshSerialize};
use openindex_sdk::{
    openindex::{
        error::ProtocolError,
        seeds::{CONTROLLER_GLOBAL_CONFIG_SEED, PROTOCOL_SEED},
    },
    require,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};
pub fn process_init_controller_global_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    max_index_components: u32,
) -> ProgramResult {
    if max_index_components == 0 {
        return Err(ProtocolError::InvalidMaxIndexComponents.into());
    }

    let accounts_iter = &mut accounts.iter();
    let owner = next_account_info(accounts_iter)?;
    let protocol_account = next_account_info(accounts_iter)?;
    let controller_global_config_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;

    require!(owner.is_signer, ProgramError::MissingRequiredSignature);

    require!(
        controller_global_config_account.lamports() == 0,
        ProgramError::AccountAlreadyInitialized
    );

    require!(
        protocol_account.owner == program_id,
        ProtocolError::UnknownProtocolAccount.into()
    );

    let protocol: Protocol = Protocol::try_from_slice(&protocol_account.data.borrow())
        .map_err(|_| ProtocolError::InvalidProtocolAccountData)?;

    require!(
        protocol.is_initialized(),
        ProtocolError::ProtocolNotInitialized.into()
    );

    let protocol_pda =
        Pubkey::create_program_address(&[&PROTOCOL_SEED, &[protocol.bump]], &program_id)?;

    require!(
        *protocol_account.key == protocol_pda,
        ProtocolError::IncorrectProtocolAccount.into()
    );

    require!(
        *owner.key == protocol.owner,
        ProtocolError::OnlyProtocolOwner.into()
    );

    let (controller_global_config_pda, controller_global_conifg_bump) =
        Pubkey::find_program_address(&[CONTROLLER_GLOBAL_CONFIG_SEED], &program_id);

    require!(
        *controller_global_config_account.key == controller_global_config_pda,
        ProtocolError::IncorrectControllerGlobalConfigAccount.into()
    );

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

    let controller_global_conifg =
        ControllerGlobalConfig::new(max_index_components, controller_global_conifg_bump);
    controller_global_conifg
        .serialize(&mut &mut controller_global_config_account.data.borrow_mut()[..])?;

    Ok(())
}
