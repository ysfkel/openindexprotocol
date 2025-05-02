use borsh::BorshDeserialize;
use openindex_sdk::{
    openindex::{
        error::ProtocolError,
        pda::{
            create_component_address, create_component_vault_address,
            create_index_mints_data_address, find_index_mint_authority_address,
        },
        seeds::COMPONENT_VAULT_SEED,
    },
    require,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::instruction::{burn, burn_checked, transfer};

use crate::state::{Component, IndexMints, Protocol};

pub fn process_redeem(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index_id: u64,
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let caller = next_account_info(accounts_iter)?;
    let controller_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority_account = next_account_info(accounts_iter)?;
    let index_account = next_account_info(accounts_iter)?;
    let index_mints_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    require!(
        caller.is_signer == true,
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

    let index_mints_data = IndexMints::try_from_slice(&index_mints_account.data.borrow()[..])
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

    for (index, mint) in mints.iter().enumerate() {
        let component_mint_account = next_account_info(accounts_iter)?;
        let component_account = next_account_info(accounts_iter)?;
        let vault_pda = next_account_info(accounts_iter)?;
        let vault_ata = next_account_info(accounts_iter)?;
        let token_account = next_account_info(accounts_iter)?;

        require!(
            token_account.owner == token_program_account.key,
            ProgramError::InvalidAccountOwner
        );

        require!(
            component_mint_account.owner == token_program_account.key,
            ProgramError::IncorrectProgramId
        );

        require!(
            component_mint_account.key == mint,
            ProtocolError::InvalidMintAccount.into()
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

        let expected_vault_ata = spl_associated_token_account::get_associated_token_address(
            vault_pda.key,
            component_mint_account.key,
        );
        require!(
            *vault_ata.key == expected_vault_ata,
            ProtocolError::IncorrectVaultATA.into()
        );

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

        let component_amount = amount
            .checked_mul(component.uints)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        invoke_signed(
            &transfer(
                token_program_account.key,
                vault_ata.key,
                token_account.key,
                vault_pda.key,
                &[],
                component_amount,
            )?,
            &[
                token_program_account.clone(),
                vault_ata.clone(),
                token_account.clone(),
                vault_pda.clone(),
            ],
            &[&[
                COMPONENT_VAULT_SEED,
                index_account.key.as_ref(),
                component_mint_account.key.as_ref(),
                &[component.vault_bump],
            ]],
        )?;
    }

    invoke(
        &burn(
            token_program_account.key,
            token_account.key,
            mint_account.key,
            &caller.key,
            &[],
            amount,
        )?,
        &[
            token_program_account.clone(),
            token_account.clone(),
            mint_account.clone(),
            caller.clone(),
        ],
    )?;
    Ok(())
}
