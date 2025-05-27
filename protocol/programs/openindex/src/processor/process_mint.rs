//! Program state processor

use crate::state::{Component, IndexMints};
use borsh::BorshDeserialize;
use openindex_sdk::{
    openindex::{
        error::ProtocolError,
        pda::{
            create_component_address, create_component_vault_address,
            create_index_mints_data_address, find_index_mint_authority_address,
        },
        seeds::INDEX_MINT_AUTHORITY_SEED,
    },
    require,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};
use spl_token::instruction::{mint_to, transfer};

//// instruction to process minting an index
pub fn process_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index_id: u64,
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let signer = next_account_info(accounts_iter)?;
    let controller_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority_account = next_account_info(accounts_iter)?;
    let index_account = next_account_info(accounts_iter)?;
    let index_mints_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    require!(
        signer.is_signer,
        ProgramError::MissingRequiredSignature
    );

    require!(
        amount > 0,
        ProtocolError::AmountMustBeGreaterThanZero.into()
    );

    require!(
        controller_account.owner == program_id,
        ProtocolError::UnknownControllerAccount.into()
    );

    require!(
        index_account.owner == program_id,
        ProtocolError::UnknownIndexAccount.into()
    );

    let index_mints_data = IndexMints::try_from_slice(&index_mints_account.data.borrow_mut()[..])
        .map_err(|_| ProtocolError::InvalidIndexMintsAccountData)?;

    let index_mints_pda = create_index_mints_data_address(
        program_id,
        controller_account.key,
        index_id,
        index_mints_data.bump,
    )?;

    require!(
        *index_mints_account.key == index_mints_pda,
        ProtocolError::IncorrectIndexMintsAccount.into()
    );

    let mints = index_mints_data.mints;

    let (mint_authority_pda, mint_authority_bump) =
        find_index_mint_authority_address(program_id, controller_account.key, index_id);

    require!(
        *mint_authority_account.key == mint_authority_pda,
        ProtocolError::IncorrectMintAuthority.into()
    );

    for _ in mints.iter() {
        let component_mint_account = next_account_info(accounts_iter)?;
        let component_account = next_account_info(accounts_iter)?;
        let vault_pda = next_account_info(accounts_iter)?;
        let vault_ata = next_account_info(accounts_iter)?;
        let component_token_account = next_account_info(accounts_iter)?;

        require!(
            component_token_account.owner == token_program_account.key,
            ProgramError::InvalidAccountOwner
        );

        require!(
            component_mint_account.owner == token_program_account.key,
            ProgramError::IncorrectProgramId
        );

        let component = Component::try_from_slice(&component_account.data.borrow_mut()[..])
            .map_err(|_| ProtocolError::InvalidComponentData)?;

        let component_pda = create_component_address(
            program_id,
            index_account.key,
            component_mint_account.key,
            component.bump,
        )?;

        require!(
            *component_account.key == component_pda,
            ProtocolError::IncorrectComponentAccount.into()
        );

        require!(
            component.is_initialized(),
            ProtocolError::ComponentNotInitialized.into()
        );

        let component_amount = amount
            .checked_mul(component.uints)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        let expected_vault_pda = create_component_vault_address(
            program_id,
            index_account.key,
            component_mint_account.key,
            component.vault_bump,
        )?;

        require!(
            *vault_pda.key == expected_vault_pda,
            ProtocolError::IncorrectVaultAccount.into()
        );


        let expected_vault_ata = spl_associated_token_account::get_associated_token_address(
            vault_pda.key,
            component_mint_account.key,
        );
        require!(
            *vault_ata.key == expected_vault_ata,
            ProtocolError::IncorrectVaultATA.into()
        );

        invoke(
            &transfer(
                token_program_account.key,
                component_token_account.key,
                vault_ata.key,
                signer.key,
                &[],
                component_amount,
            )?,
            &[
                signer.clone(),
                component_token_account.clone(),
                vault_ata.clone(),
                token_program_account.clone(),
            ],
        )?;
    }
    let token_account_data = spl_token::state::Account::unpack(&token_account.data.borrow())?;
    require!(
        *mint_account.key == token_account_data.mint,
        ProtocolError::InvalidMintAccount.into()
    );

    invoke_signed(
        &mint_to(
            token_program_account.key,
            mint_account.key,
            token_account.key,
            &mint_authority_pda,
            &[],
            amount,
        )?,
        &[
            token_program_account.clone(),
            mint_account.clone(),
            token_account.clone(),
            mint_authority_account.clone(),
        ],
        &[&[
            INDEX_MINT_AUTHORITY_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
            &[mint_authority_bump],
        ]],
    )?;

    Ok(())
}
