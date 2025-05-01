use crate::state::Module;
use borsh::{BorshDeserialize, BorshSerialize};
use openindex_sdk::{
    openindex::{
        error::ProtocolError,
        seeds::{INDEX_MINT_AUTHORITY_SEED, INDEX_MINT_SEED, MODULE_SEED},
    },
    require,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::instruction::mint_to;
pub fn mint_index(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index_id: u64,
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let caller = next_account_info(accounts_iter)?;
    let registered_module_account = next_account_info(accounts_iter)?;
    let controller_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    require!(caller.is_signer, ProgramError::MissingRequiredSignature);
    require!(
        registered_module_account.owner == program_id,
        ProtocolError::UnknownModuleAccount.into()
    );

    let (registered_module_pda, _) =
        Pubkey::find_program_address(&[&MODULE_SEED, &caller.key.as_ref()], program_id);
    require!(
        *registered_module_account.key == registered_module_pda,
        ProtocolError::IncorrectModuleAccount.into()
    );

    let mut registered_module: Module =
        Module::try_from_slice(&registered_module_account.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)?;

    require!(
        registered_module.is_active(),
        ProtocolError::OnlyActiveModules.into()
    );

    require!(
        controller_account.owner == program_id,
        ProtocolError::UnknownControllerAccount.into()
    );

    require!(
        *token_program_account.key == *token_program_account.key,
        ProgramError::IncorrectProgramId
    );

    require!(
        mint_account.owner == token_program_account.key,
        ProgramError::IncorrectProgramId
    );

    // check mint
    let (mint_pda, _) = Pubkey::find_program_address(
        &[
            INDEX_MINT_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
        ],
        program_id,
    );

    require!(
        *mint_account.key == mint_pda,
        ProtocolError::IncorrectMintAccount.into()
    );

    let token_account_data = spl_token::state::Account::unpack(&token_account.try_borrow_data()?)?;
    require!(
        token_account_data.mint == *mint_account.key,
        ProtocolError::InvalidTokenAccount.into()
    );

    require!(
        token_account.owner == token_program_account.key,
        ProgramError::IncorrectProgramId
    );

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
