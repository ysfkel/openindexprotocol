use crate::state::{Component, IndexMints};
use borsh::BorshDeserialize;
use openindex_sdk::{
    openindex::{
        error::ProtocolError,
        seeds::{
            COMPONENT_SEED, COMPONENT_VAULT_SEED, INDEX_MINTS_DATA_SEED, INDEX_MINT_AUTHORITY_SEED,
        },
    },
    require,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_token::instruction::{mint_to, transfer};

pub fn mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index_id: u64,
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let caller_account = next_account_info(accounts_iter)?;
    let controller_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority_account = next_account_info(accounts_iter)?;
    let index_account = next_account_info(accounts_iter)?;
    let index_mints_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    require!(
        caller_account.is_signer,
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

    let index_mints_pda = Pubkey::create_program_address(
        &[
            INDEX_MINTS_DATA_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
            &[index_mints_data.bump],
        ],
        program_id,
    )?;

    require!(
        *index_mints_account.key == index_mints_pda,
        ProtocolError::IncorrectIndexMintsAccount.into()
    );

    let mints = index_mints_data.mints;

    let (mint_authority_pda, mint_authority_bump) = Pubkey::find_program_address(
        &[
            INDEX_MINT_AUTHORITY_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
        ],
        program_id,
    );
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

        let component = Component::try_from_slice(&component_account.data.borrow_mut()[..])
            .map_err(|_| ProtocolError::InvalidComponentData)?;

        let component_pda = Pubkey::create_program_address(
            &[
                COMPONENT_SEED,
                index_account.key.as_ref(),
                component_mint_account.key.as_ref(),
                &[component.bump],
            ],
            program_id,
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

        require!(
            component.is_initialized(),
            ProtocolError::ComponentNotInitialized.into()
        );

        let expected_vault_pda = Pubkey::create_program_address(
            &[
                COMPONENT_VAULT_SEED,
                index_account.key.as_ref(),
                component_mint_account.key.as_ref(),
                &[component.vault_bump],
            ],
            program_id,
        )?;

        require!(
            *vault_pda.key == expected_vault_pda,
            ProtocolError::IncorrectVaultAccount.into()
        );

        let component_amount = amount
            .checked_mul(component.uints)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        invoke(
            &transfer(
                token_program_account.key,
                token_account.key,
                vault_ata.key,
                caller_account.key,
                &[],
                component_amount,
            )?,
            &[
                caller_account.clone(),
                token_account.clone(),
                vault_ata.clone(),
                token_program_account.clone(),
            ],
        )?;
    }

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
