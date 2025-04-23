use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{entrypoint, ProgramResult},
    instruction::{AccountMeta, Instruction},
    loader_v4::transfer_authority,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
};

use crate::{
    error::ProtocolError,
    require,
    state::{Component, IndexMints},
};
use open_index_lib::{
    instruction::ProtocolInstruction,
    seeds::{COMPONENT_SEED, COMPONENT_VAULT_SEED, INDEX_MINTS_DATA_SEED, MODULE_SEED},
};
use spl_token::instruction::transfer;
pub fn mint_index(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index_id: u64,
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let caller_account = next_account_info(accounts_iter)?;
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
        amount > 0,
        ProtocolError::AmountMustBeGreaterThanZero.into(),
        "amount must be greater than 0"
    );

    require!(
        caller_account.is_signer,
        ProgramError::MissingRequiredSignature,
        "caller must be signer"
    );

    require!(
        module_account.owner == program_id,
        ProtocolError::UnknownModuleAccount.into(),
        "program does not own module"
    );

    let (signer_pda, bump) = Pubkey::find_program_address(&[program_id.as_ref()], program_id);
    require!(
        *module_account.key == signer_pda,
        ProtocolError::IncorrectModuleAccount.into(),
        "incorrect module account"
    );

    let (registered_module_pda, registered_module_bump) = Pubkey::find_program_address(
        &[&MODULE_SEED, &module_account.key.as_ref()],
        open_index_account.key,
    );

    require!(
        *registered_module_account.key == registered_module_pda,
        ProtocolError::InvalidRegisredModuleAccount.into(),
        "invalid registered module account"
    );

    //TODO validate open_index_account, controller_account

    // Get Index Mints
    let (index_mints_pda, index_mints_bump) = Pubkey::find_program_address(
        &[
            INDEX_MINTS_DATA_SEED,
            controller_account.key.as_ref(),
            &index_id.to_le_bytes(),
        ],
        open_index_account.key,
    );

    require!(
        *index_mints_account.key == index_mints_pda,
        ProtocolError::IncorrectIndexMintsAccount.into(),
        "incorrect index mints account"
    );

    let index_mints =
        IndexMints::try_from_slice(&index_mints_account.data.borrow()).map_err(|_| {
            msg!("Failed to deserialize index_mints data");
            ProgramError::InvalidAccountData
        })?;
    let mints = index_mints.mints;

    for (index, mint) in mints.iter().enumerate() {
        let component_mint_account = next_account_info(accounts_iter)?;
        let component_account = next_account_info(accounts_iter)?;
        let vault_pda = next_account_info(accounts_iter)?;
        let vault_ata = next_account_info(accounts_iter)?;

        require!(
            component_mint_account.owner == token_program_account.key,
            ProgramError::IncorrectProgramId,
            "token program does not own mint account"
        );

        require!(
            component_mint_account.key == mint,
            ProtocolError::InvalidMintAccount.into(),
            "invalid mint account"
        );

        // Get component
        let (component_pda, component_bump) = Pubkey::find_program_address(
            &[
                COMPONENT_SEED,
                index_account.key.as_ref(),
                component_mint_account.key.as_ref(),
            ],
            program_id,
        );
        require!(
            *component_account.key == component_pda,
            ProtocolError::IncorrectComponentAccount.into(),
            "incorrect component account"
        );

        //Get vault

        let expected_vault_ata = spl_associated_token_account::get_associated_token_address(
            vault_pda.key,
            component_mint_account.key,
        );
        require!(
            *vault_ata.key == expected_vault_ata,
            ProtocolError::IncorrectVaultATA.into(),
            "incorrect vault ata"
        );
        let component = Component::try_from_slice(&component_account.data.borrow_mut()[..])
            .map_err(|_| {
                msg!("Failed to deserialize component data ");
                ProgramError::InvalidAccountData
            })?;

        require!(
            component.is_initialized(),
            ProtocolError::ComponentNotInitialized.into(),
            format!("component not initialized {:?}", component_account.key).as_str()
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
            ProtocolError::IncorrectVaultAccount.into(),
            "incorrect vault account"
        );

        let component_amount = amount
            .checked_mul(component.uints)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        invoke(
            &transfer(
                token_program_account.key,
                caller_account.key,
                vault_ata.key,
                caller_account.key,
                &[],
                component_amount,
            )?,
            &[
                caller_account.clone(),
                vault_ata.clone(),
                token_program_account.clone(),
            ],
        )?;
    }

    // Mint index token
    let initialize_ix = & ProtocolInstruction::Mint { amount, index_id };
    let mut initialize_ix_data = Vec::new();
    initialize_ix.serialize(&mut initialize_ix_data).unwrap();

    let cpi_accounts = vec![
        AccountMeta::new_readonly(module_account.key.clone(), true),
        AccountMeta::new_readonly(registered_module_account.key.clone(), false),
        AccountMeta::new_readonly(controller_account.key.clone(), false),
        AccountMeta::new_readonly(mint_account.key.clone(), false),
        AccountMeta::new_readonly(mint_authority_account.key.clone(), false),
        AccountMeta::new(token_account.key.clone(), false),
        AccountMeta::new_readonly(token_program_account.key.clone(), false),
    ];

    let cpi_instruction = Instruction {
        program_id: *open_index_account.key,
        accounts: cpi_accounts,
        data: initialize_ix_data,
    };

    invoke_signed(
        &cpi_instruction,
        &[module_account.clone(), registered_module_account.clone()], // Pass the actual AccountInfo references
        &[&[program_id.as_ref(), &[bump]]],
    )?;

    msg!("open_index invoked");

    Ok(())
}
