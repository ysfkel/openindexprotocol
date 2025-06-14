//! Program state processor
use borsh::{BorshDeserialize, BorshSerialize};
use openindex_sdk::{
    openindex::{
        state::{Component, Controller, ControllerGlobalConfig, Index, IndexMints},
        error::ProtocolError,
        pda::{
            create_index_address, find_component_address, find_component_vault_address,
             find_index_mints_data_address,
        },
        seeds::{COMPONENT_SEED, COMPONENT_VAULT_SEED, INDEX_MINTS_DATA_SEED},
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
    system_instruction,
    sysvar::Sysvar,
};
use spl_associated_token_account::instruction::create_associated_token_account;

/// instruction to process adding index components
pub fn process_add_index_components(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    mints: Vec<Pubkey>,
    amounts: Vec<u64>,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let signer = next_account_info(accounts_iter)?;
    let index_account = next_account_info(accounts_iter)?;
    let index_mints_account = next_account_info(accounts_iter)?;
    let controller_account = next_account_info(accounts_iter)?;
    let controller_global_config_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;
    let associated_token_program_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    require!(signer.is_signer, ProgramError::MissingRequiredSignature);

    require!(
        index_account.owner == program_id,
        ProtocolError::UnknownIndexAccount.into()
    );

    require!(
        controller_global_config_account.owner == program_id,
        ProtocolError::UnknownControllerGlobalConfigAccount.into()
    );

    require!(
        controller_account.owner == program_id,
        ProtocolError::UnknownControllerAccount.into()
    );

    let controller = Controller::try_from_slice(&controller_account.data.borrow())?;
    require!(
        controller.owner == *signer.key,
        ProtocolError::OnlyControllerOwner.into()
    );

    let controller_global_config =
        ControllerGlobalConfig::try_from_slice(&controller_global_config_account.data.borrow())?;
    require!(
        controller_global_config.is_initialized(),
        ProtocolError::ControllerGlobalConfigNotInitialized.into()
    );

    let index_data = Index::try_from_slice(&index_account.data.borrow())?;
    let index_id = index_data.id;

    let index_pda = create_index_address(
        program_id,
        controller_account.key,
        index_id,
        index_data.bump,
    )?;

    require!(
        *index_account.key == index_pda,
        ProtocolError::IncorrectIndexAccount.into()
    );

    let index_data: Index = Index::try_from_slice(&index_account.data.borrow())?;

    require!(
        index_data.is_initialized(),
        ProtocolError::IndexNotInitialized.into()
    );

    let (index_mints_pda, index_mints_bump) =
        find_index_mints_data_address(program_id, controller_account.key, index_id);

    require!(
        *index_mints_account.key == index_mints_pda,
        ProtocolError::IncorrectIndexMintsAccount.into()
    );

    let mints_len = mints.len();
    require!(mints_len > 0, ProtocolError::NoMintsProvided.into());

    require!(
        mints_len <= controller_global_config.max_index_components as usize,
        ProtocolError::MaxIndexComponentsExceeded.into()
    );

    require!(
        mints_len == amounts.len(),
        ProtocolError::MintsAmountsLenMismatch.into()
    );



    // creates components
    let rent = Rent::get()?;
    let component_lamports = rent.minimum_balance(Component::LEN);
    for (index, mint) in mints.iter().enumerate() {
        let mint_account = next_account_info(accounts_iter)?;
        let component_account = next_account_info(accounts_iter)?;
        let vault_pda = next_account_info(accounts_iter)?;
        let vault_ata = next_account_info(accounts_iter)?;

        require!(
            mint_account.owner == token_program_account.key,
            ProgramError::IncorrectProgramId
        );

        require!(
            mint_account.key == mint,
            ProtocolError::InvalidMintAccount.into()
        );

        let amount = amounts
            .get(index)
            .ok_or(ProtocolError::ComponentAmountError)?;

        let (component_pda, component_bump) =
            find_component_address(program_id, index_account.key, mint_account.key);

        require!(
            *component_account.key == component_pda,
            ProtocolError::IncorrectComponentAccount.into()
        );

        let (expected_vault_pda, vault_bump) =
            find_component_vault_address(program_id, index_account.key, mint_account.key);

        require!(
            *vault_pda.key == expected_vault_pda,
            ProtocolError::IncorrectVaultAccount.into()
        );

        let expected_vault_ata = spl_associated_token_account::get_associated_token_address(
            vault_pda.key,
            mint_account.key,
        );
        require!(
            *vault_ata.key == expected_vault_ata,
            ProtocolError::IncorrectVaultATA.into()
        );

        // create component account
        invoke_signed(
            &system_instruction::create_account(
                &signer.key,
                &component_account.key,
                component_lamports,
                Component::LEN as u64,
                program_id,
            ),
            &[
                signer.clone(),
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

        // Initialize component data
        let component = Component::new(
            *amount,
            mint_account.key.clone(),
            component_bump,
            vault_bump,
        );
        component.serialize(&mut &mut component_account.data.borrow_mut()[..])?;
        // create vault associated token account
        invoke_signed(
            &create_associated_token_account(
                signer.key,
                vault_pda.key,
                mint_account.key,
                &spl_token::ID,
            ),
            &[
                signer.clone(),
                vault_ata.clone(),
                vault_pda.clone(),
                mint_account.clone(),
                system_program_account.clone(),
                token_program_account.clone(),
                associated_token_program_account.clone(),
            ],
            &[&[
                COMPONENT_VAULT_SEED,
                index_account.key.as_ref(),
                mint_account.key.as_ref(),
                &[vault_bump],
            ]],
        )?;
    }
    
    //creates index mints account
    let space = IndexMints::calc_len(mints_len);
    let lamports = rent.minimum_balance(space);

    invoke_signed(
        &system_instruction::create_account(
            &signer.key,
            &index_mints_account.key,
            lamports,
            space as u64,
            program_id,
        ),
        &[
            signer.clone(),
            index_mints_account.clone(),
            system_program_account.clone(),
        ],
        &[&[
            INDEX_MINTS_DATA_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
            &[index_mints_bump],
        ]],
    )?;

    let index_mints = IndexMints::new(mints, index_mints_bump);
    index_mints.serialize(&mut &mut index_mints_account.data.borrow_mut()[..])?;

    Ok(())
}
