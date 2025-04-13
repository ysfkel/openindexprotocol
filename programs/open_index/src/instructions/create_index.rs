use crate::{
    error::ProtocolError,
    instructions::mint_index,
    require,
    seeds::{COMPONENT_SEED, INDEX_MINT_AUTHORITY_SEED, INDEX_MINT_SEED, INDEX_SEED, VAULT_SEED},
    state::{Component, Controller, ControllerGlobalConfig, Index, Protocol},
};
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
use spl_token::{
    instruction::{initialize_mint, initialize_mint2},
    state::Mint,
};

pub fn create_index(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let owner = next_account_info(accounts_iter)?;
    let manager = next_account_info(accounts_iter)?;
    let index_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let controller_account = next_account_info(accounts_iter)?;
    let controller_global_config_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    require!(
        owner.is_signer,
        ProgramError::MissingRequiredSignature,
        "owner must be signer"
    );
    require!(
        index_account.lamports() == 0,
        ProgramError::AccountAlreadyInitialized,
        "index initialized"
    );

    require!(
        controller_global_config_account.owner == program_id,
        ProtocolError::UnknownControllerGlobalConfigAccount.into(),
        "program does not own controller global config account"
    );

    require!(
        controller_account.owner == program_id,
        ProtocolError::UnknownControllerAccount.into(),
        "program does not own controller account"
    );
    
    let mut controller = Controller::try_from_slice(&controller_account.data.borrow())?;
    require!(
        controller.owner == *owner.key,
        ProtocolError::OnlyControllerOwner.into(),
        "only controller owner can execute this instruction"
    );

    ///
    let controller_global_config =
        ControllerGlobalConfig::try_from_slice(&controller_global_config_account.data.borrow())?;
    require!(
        controller_global_config.is_initialized(),
        ProtocolError::ControllerGlobalConfigNotInitialized.into(),
        "controller global config not initialized"
    );

    let index_id = controller.get_next_index_id();
    let (index_pda, index_bump) = Pubkey::find_program_address(
        &[
            INDEX_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
        ],
        program_id,
    );

    require!(
        *index_account.key == index_pda,
        ProtocolError::IncorrectIndexAccount.into(),
        "incorrect index account"
    );

    //
    let (mint_pda, mint_bump) = Pubkey::find_program_address(
        &[
            INDEX_MINT_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
        ],
        program_id,
    );
    require!(
        mint_account.key == &mint_pda,
        ProtocolError::IncorrectMintAccount.into(),
        "incorrect mint account"
    );

    require!(
        *system_program_account.key == system_program::ID,
        ProgramError::IncorrectProgramId,
        "invalid system program"
    );

    let rent = Rent::get()?;

    /// Create Index
    let space = Index::LEN;
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

    // Create Token Mint
    let mint_space = Mint::LEN;
    let mint_lamports = rent.minimum_balance(mint_space);
    invoke_signed(
        &system_instruction::create_account(
            owner.key,
            mint_account.key,
            mint_lamports,
            mint_space as u64,
            program_id,
        ),
        &[
            owner.clone(),
            mint_account.clone(),
            system_program_account.clone(),
        ],
        &[&[
            INDEX_MINT_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
            &[mint_bump],
        ]],
    )?;

    // Initialize Mint
    let (mint_authority_pda, mint_authority_bump) = Pubkey::find_program_address(
        &[
            INDEX_MINT_AUTHORITY_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
        ],
        program_id,
    );

    require!(
        mint_account.owner == token_program_account.key,
        ProgramError::IncorrectProgramId,
        "mint account not owned by token program"
    );
    
    invoke_signed(
        &initialize_mint2(
            &token_program_account.key,
            mint_account.key,
            &mint_authority_pda,
            Some(&mint_authority_pda),
            9,
        )?,
        &[mint_account.clone(), token_program_account.clone()],
        &[&[
            INDEX_MINT_AUTHORITY_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
            &[mint_authority_bump],
        ]],
    )?;
    msg!("mint account initialized {:?}", mint_account.key);

    // Serialize Data
    let index = Index::new(index_id, owner.key.clone(), manager.key.clone(), index_bump);
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
