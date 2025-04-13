use crate::{
    error::ProtocolError,
    require,
    seeds::{INDEX_MINT_AUTHORITY_SEED, INDEX_MINT_SEED, MODULE_SEED},
    state::Module,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    entrypoint::ProgramResult,
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

    require!(
        caller.is_signer,
        ProgramError::MissingRequiredSignature,
        "caller must be signer"
    );
    require!(
        registered_module_account.owner == program_id,
        ProtocolError::UnknownModuleAccount.into(),
        "program does not own module"
    );

    let (registered_module_pda, _) =
        Pubkey::find_program_address(&[&MODULE_SEED, &caller.key.as_ref()], program_id);
    require!(
        *registered_module_account.key == registered_module_pda,
        ProtocolError::IncorrectModuleAccount.into(),
        "invalid module"
    );

    let mut registered_module: Module =
        Module::try_from_slice(&registered_module_account.data.borrow()).map_err(|_| {
            msg!("Failed to deserialize registered_module_account data");
            ProgramError::InvalidAccountData
        })?;

    require!(
        registered_module.active,
        ProtocolError::OnlyActiveModules.into(),
        "only active modules can mint"
    );

    require!(
        controller_account.owner == program_id,
        ProtocolError::UnknownControllerAccount.into(),
        "program does not own controller"
    );

    require!(
        *token_program_account.key == spl_token::ID,
        ProgramError::IncorrectProgramId,
        "invalid token program"
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

    let token_account_data = spl_token::state::Account::unpack(&token_account.try_borrow_data()?)?;
    require!(
        token_account_data.mint == *mint_account.key,
        ProtocolError::InvalidTokenAccount.into(),
        "Token account is not associated with the correct mint"
    );
    // Mint
    require!(
        *mint_account.key == mint_pda,
        ProtocolError::IncorrectMintAccount.into(),
        "incorrect mint account"
    );

    require!(
        mint_account.owner == token_program_account.key,
        ProgramError::IncorrectProgramId,
        "mint account not owned by token program"
    );

    require!(
        token_account.owner == token_program_account.key,
        ProgramError::IncorrectProgramId,
        "token account not owned by token program"
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
        ProtocolError::IncorrectMintAuthority.into(),
        "incorrect mint authority account"
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
            mint_account.clone(),
            token_account.clone(),
            mint_authority_account.clone(),
            token_program_account.clone(),
        ],
        &[&[
            INDEX_MINT_AUTHORITY_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
            &[mint_authority_bump],
        ]],
    )?;

    msg!(
        "Minted {} tokens to account: {:?}",
        amount,
        token_account.key
    );

    Ok(())
}
