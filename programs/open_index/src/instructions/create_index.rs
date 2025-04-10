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
    seeds::{COMPONENT_SEED, INDEX_SEED},
    state::{Component, Controller, ControllerGlobalConfig, Index, Protocol},
};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CreateIndex {
    pub mints: Vec<Pubkey>,
    pub amounts: Vec<u64>,
}

pub fn create_index(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    mints: Vec<Pubkey>,
    amounts: Vec<u64>,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let owner = next_account_info(accounts_iter)?;
    let manager = next_account_info(accounts_iter)?;
    let index_account = next_account_info(accounts_iter)?;
    let controller_account = next_account_info(accounts_iter)?;
    let controller_global_config_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;

    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if index_account.lamports() > 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if controller_global_config_account.owner != program_id {
        return Err(ProtocolError::UnknownControllerGlobalConfigAccount.into());
    }

    if controller_account.owner != program_id {
        return Err(ProtocolError::UnknownControllerAccount.into());
    }

    let mut controller = Controller::try_from_slice(&controller_account.data.borrow())?;

    if controller.owner != *owner.key {
        return Err(ProtocolError::OnlyControllerOwnerCanExecuteThisInstruction.into());
    }

    if *system_program_account.key != system_program::id() {
        return Err(ProtocolError::InvalidSystemProgramAccount.into());
    }

    let controller_global_config =
        ControllerGlobalConfig::try_from_slice(&controller_global_config_account.data.borrow())?;

    if !controller_global_config.is_initialized() {
        return Err(ProtocolError::ControllerGlobalConfigNotInitialized.into());
    }

    let index_id = controller.get_next_index_id();

    let (index_pda, index_bump) = Pubkey::find_program_address(
        &[
            INDEX_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
        ],
        program_id,
    );

    if *index_account.key != index_pda {
        return Err(ProtocolError::IncorrectIndexAccount.into());
    }

    let mints_len = mints.len();

    if mints_len == 0 {
        return Err(ProtocolError::NoMintsProvided.into());
    }

    if mints_len > controller_global_config.max_index_components as usize {
        return Err(ProtocolError::MaxIndexComponentsExceeded.into());
    }

    if mints_len != amounts.len() {
        return Err(ProtocolError::MintsAmountsLenMismatch.into());
    }

    let space = Index::len(mints_len);
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space);

    // create index
    // seed index + controller_key + index_id
    invoke_signed(
        &system_instruction::create_account(
            &owner.key,
            &index_account.key,
            lamports,
            space as u64,
            program_id,
        ),
        &[
            owner.clone(),
            index_account.clone(),
            system_program_account.clone(),
        ],
        &[&[
            INDEX_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
            &[index_bump],
        ]],
    )?;

    msg!("index created {:?}", index_account.key);
    msg!("initializing index components");
    let component_lamports = rent.minimum_balance(Component::LEN);
    // creates components
    for (index, mint) in mints.iter().enumerate() {
        let mint_account = next_account_info(accounts_iter)?;
        let component_account = next_account_info(accounts_iter)?;

        if *mint_account.owner != spl_token::id() {
            return Err(ProtocolError::InvalidTokenMint.into());
        }

        if mint_account.key != mint {
            return Err(ProtocolError::InvalidMintAccount.into());
        }

        let amount = amounts
            .get(index)
            .ok_or(ProtocolError::ComponentAmountError)?;

        let (component_pda, component_bump) = Pubkey::find_program_address(
            &[
                COMPONENT_SEED,
                controller_account.key.as_ref(),
                index_account.key.as_ref(),
                mint_account.key.as_ref(),
            ],
            program_id,
        );

        if *component_account.key != component_pda {
            return Err(ProtocolError::IncorrectComponentAccount.into());
        }



        invoke_signed(
            &system_instruction::create_account(
                &owner.key,
                &component_account.key,
                component_lamports,
                Component::LEN as u64,
                program_id,
            ),
            &[
                owner.clone(),
                component_account.clone(),
                system_program_account.clone(),
            ],
            &[&[
                COMPONENT_SEED,
                controller_account.key.as_ref(),
                index_account.key.as_ref(),
                mint_account.key.as_ref(),
                &[component_bump],
            ]],
        )?;

        let component = Component::new(*amount, mint_account.key.clone(), component_bump);
        msg!("serializing component data for mint {:?}", mint_account.key);
        component.serialize(&mut &mut component_account.data.borrow_mut()[..])?;
        msg!("component created {:?}", component_account.key);
    }

    let index = Index::new(
        index_id,
        owner.key.clone(),
        manager.key.clone(),
        mints,
        index_bump,
    );

    msg!("serializing index data {:?}", index_account.key);
    index.serialize(&mut &mut index_account.data.borrow_mut()[..])?;
    msg!("index initialized {:?}", index_account.key);

    controller.increment_next_index_id();
    let next_id = controller.get_next_index_id();

    msg!("serializing controller data {:?}", controller_account.key);
    controller.serialize(&mut &mut controller_account.data.borrow_mut()[..])?;
    msg!("controller updated  {:?} next_index_id {:?}", controller_account.key, next_id);
    
    Ok(())
}
