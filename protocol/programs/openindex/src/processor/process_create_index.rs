//! Program state processor
use borsh::{BorshDeserialize, BorshSerialize};
use openindex_sdk::{
    openindex::{
        error::ProtocolError,
        pda::{find_index_address, find_index_mint_address},
        seeds::{INDEX_MINT_AUTHORITY_SEED, INDEX_MINT_SEED, INDEX_SEED},
        state::{Controller, ControllerGlobalConfig, Index},
    },
    require,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};
use spl_token::{instruction::initialize_mint2, state::Mint};

/// instruction to process creating an index
pub fn process_create_index(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let signer = next_account_info(accounts_iter)?;
    let manager = next_account_info(accounts_iter)?;
    let index_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let controller_account = next_account_info(accounts_iter)?;
    let controller_global_config_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    require!(signer.is_signer, ProgramError::MissingRequiredSignature);
    require!(
        index_account.lamports() == 0,
        ProgramError::AccountAlreadyInitialized
    );

    require!(
        mint_account.lamports() == 0,
        ProgramError::AccountAlreadyInitialized
    );

    require!(
        controller_global_config_account.owner == program_id,
        ProtocolError::UnknownControllerGlobalConfigAccount.into()
    );

    require!(
        controller_account.owner == program_id,
        ProtocolError::UnknownControllerAccount.into()
    );

    let mut controller = Controller::try_from_slice(&controller_account.data.borrow())?;
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

    let index_id = controller.get_next_index_id();

    let (index_pda, index_bump) = find_index_address(program_id, controller_account.key, index_id);

    require!(
        *index_account.key == index_pda,
        ProtocolError::IncorrectIndexAccount.into()
    );

    let (mint_pda, mint_bump) =
        find_index_mint_address(program_id, controller_account.key, index_id);

    require!(
        *mint_account.key == mint_pda,
        ProtocolError::IncorrectMintAccount.into()
    );

    require!(
        *system_program_account.key == system_program::ID,
        ProgramError::IncorrectProgramId
    );

    require!(
        token_program_account.key == &spl_token::id(),
        ProgramError::IncorrectProgramId
    );

    let rent = Rent::get()?;

    // Create Index
    let space = Index::LEN;
    let lamports = rent.minimum_balance(space);
    invoke_signed(
        &system_instruction::create_account(
            &signer.key,
            &index_account.key,
            lamports,
            space as u64,
            program_id,
        ),
        &[
            signer.clone(),
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
            signer.key,
            mint_account.key,
            mint_lamports,
            mint_space as u64,
            token_program_account.key,
        ),
        &[
            signer.clone(),
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

    let index = Index::new(
        index_id,
        signer.key.clone(),
        manager.key.clone(),
        index_bump,
    );
    index.serialize(&mut &mut index_account.data.borrow_mut()[..])?;

    controller.generate_next_index_id();
    controller.serialize(&mut &mut controller_account.data.borrow_mut()[..])?;

    Ok(())
}
