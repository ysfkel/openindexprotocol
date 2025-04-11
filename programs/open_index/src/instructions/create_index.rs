use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};
use spl_associated_token_account::instruction::create_associated_token_account;
use crate::{
    error::ProtocolError, require, seeds::{COMPONENT_SEED, INDEX_SEED, VAULT_SEED}, state::{Component, Controller, ControllerGlobalConfig, Index, Protocol}
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
    let associated_token_program_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    require!(owner.is_signer, ProgramError::MissingRequiredSignature);
    require!(index_account.lamports() == 0 ,ProgramError::AccountAlreadyInitialized);
    require!(controller_global_config_account.owner == program_id, ProtocolError::UnknownControllerGlobalConfigAccount.into());
    require!(controller_account.owner == program_id, ProtocolError::UnknownControllerAccount.into());

    let mut controller = Controller::try_from_slice(&controller_account.data.borrow())?;
    require!(controller.owner == *owner.key, ProtocolError::OnlyControllerOwnerCanExecuteThisInstruction.into());
    
    let controller_global_config =ControllerGlobalConfig::try_from_slice(&controller_global_config_account.data.borrow())?;
    require!(controller_global_config.is_initialized(),ProtocolError::ControllerGlobalConfigNotInitialized.into());
    

    let index_id = controller.get_next_index_id();

    let (index_pda, index_bump) = Pubkey::find_program_address(
        &[
            INDEX_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
        ],
        program_id,
    );

    require!(*index_account.key == index_pda,ProtocolError::IncorrectIndexAccount.into());

    let mints_len = mints.len();
    require!(mints_len > 0,ProtocolError::NoMintsProvided.into());
    require!(mints_len <= controller_global_config.max_index_components as usize,ProtocolError::MaxIndexComponentsExceeded.into());
    require!(mints_len == amounts.len(), ProtocolError::MintsAmountsLenMismatch.into());
    

    let space = Index::len(mints_len);
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space);

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
        let vault_pda = next_account_info(accounts_iter)?;
        let vault_ata = next_account_info(accounts_iter)?;

        require!(*mint_account.owner == spl_token::ID, ProtocolError::InvalidTokenMint.into());
        require!(mint_account.key == mint, ProtocolError::InvalidMintAccount.into());
        
        let amount = amounts
            .get(index)
            .ok_or(ProtocolError::ComponentAmountError)?;
        /// Get component PDA
        let (component_pda, component_bump) = Pubkey::find_program_address(
            &[
                COMPONENT_SEED,
                index_account.key.as_ref(),
                mint_account.key.as_ref(),
            ],
            program_id,
        );
        require!(*component_account.key == component_pda, ProtocolError::IncorrectComponentAccount.into());
        /// Get Vault PDA
        let (expected_vault_pda, vault_bump) = Pubkey::find_program_address(
            &[
                VAULT_SEED,
                index_account.key.as_ref(),
                mint_account.key.as_ref(),
            ],
            program_id,
        );
        require!(*vault_pda.key == expected_vault_pda,ProtocolError::IncorrectVaultAccount.into());
    
        let expected_vault_ata = spl_associated_token_account::get_associated_token_address(
            vault_pda.key,
            mint_account.key,
        );
        require!(*vault_ata.key == expected_vault_ata, ProtocolError::IncorrectVaultATA.into());
        
        /// create component account
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
                index_account.key.as_ref(),
                mint_account.key.as_ref(),
                &[component_bump],
            ]],
        )?;
        
        /// Initialize component data
        let component = Component::new(
            *amount,
            mint_account.key.clone(),
            component_bump,
            vault_bump,
        );
        component.serialize(&mut &mut component_account.data.borrow_mut()[..])?;
        msg!("component {:?} created {:?}", index, component_account.key);

        /// create vault associated token account  
        invoke_signed(
            &create_associated_token_account(
                owner.key,
                vault_pda.key,
                mint_account.key,
                &spl_token::ID,
            ),
            &[
                owner.clone(),
                vault_ata.clone(),
                vault_pda.clone(),
                mint_account.clone(),
                system_program_account.clone(),
                token_program_account.clone(),
                associated_token_program_account.clone(),
            ],
            &[&[
                VAULT_SEED,
                index_account.key.as_ref(),
                mint_account.key.as_ref(),
                &[vault_bump],
            ]],
        )?;
        msg!("vault {:?} associated token account created {:?}",index, vault_ata.key);
    }

    let index = Index::new(
        index_id,
        owner.key.clone(),
        manager.key.clone(),
        mints,
        index_bump,
    );

    index.serialize(&mut &mut index_account.data.borrow_mut()[..])?;
    msg!("index initialized {:?}", index_account.key);

    controller.increment_next_index_id();
    let next_id = controller.get_next_index_id();

    controller.serialize(&mut &mut controller_account.data.borrow_mut()[..])?;
    msg!(
        "controller updated  {:?} next_index_id {:?}",
        controller_account.key,
        next_id
    );

    Ok(())
}
