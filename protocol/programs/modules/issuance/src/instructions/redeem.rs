use borsh::BorshDeserialize;
use openindex::state::IndexMints;
use openindex_sdk::openindex::seeds::{INDEX_MINTS_DATA_SEED, MODULE_SEED};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{error::IssuanceError, require};

pub fn redeem(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index_id: u64,
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let caller = next_account_info(accounts_iter)?;
    let module_account = next_account_info(accounts_iter)?;
    let registered_module_account = next_account_info(accounts_iter)?;
    let controller_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority_account = next_account_info(accounts_iter)?;
    let index_account = next_account_info(accounts_iter)?;
    let index_mints_account = next_account_info(accounts_iter)?;
    let open_index_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    require!(
        caller.is_signer,
        ProgramError::MissingRequiredSignature,
        "caller must be signer"
    );

    require!(
        amount > 0,
        IssuanceError::AmountMustBeGreaterThanZero.into(),
        "amount must be greater than 0"
    );

    let (signer_pda, bump) = Pubkey::find_program_address(&[program_id.as_ref()], program_id);
    require!(
        *module_account.key == signer_pda,
        IssuanceError::IncorrectModuleAccount.into(),
        "incorrect module account"
    );

    let (registered_module_pda, registered_module_bump) = Pubkey::find_program_address(
        &[&MODULE_SEED, &module_account.key.as_ref()],
        open_index_account.key,
    );

    require!(
        *registered_module_account.key == registered_module_pda,
        IssuanceError::InvalidRegisredModuleAccount.into(),
        "invalid registered module account"
    );

    let index_mints_data = IndexMints::try_from_slice(&index_mints_account.data.borrow_mut()[..])
        .map_err(|_| {
        msg!("Failed to deserialize index_mints_account data ");
        ProgramError::InvalidAccountData
    })?;

    let index_mints_pda = Pubkey::create_program_address(
        &[
            INDEX_MINTS_DATA_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
            &[index_mints_data.bump],
        ],
        open_index_account.key,
    )?;

    require!(
        *index_mints_account.key == index_mints_pda,
        IssuanceError::IncorrectIndexMintsAccount.into(),
        "incorrect index mints account"
    );

    let index_mints =
        IndexMints::try_from_slice(&index_mints_account.data.borrow()).map_err(|_| {
            msg!("Failed to deserialize index_mints data");
            ProgramError::InvalidAccountData
        })?;
    let mints = index_mints.mints;

    for (index, mint) in mints.iter().enumerate() {}

    Ok(())
}
