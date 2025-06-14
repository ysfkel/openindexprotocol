use borsh::{BorshDeserialize, BorshSerialize};
use openindex_sdk::{
    issuance::{
        error::IssuanceError, instruction::ExecuteHookInstruction,
        pda::find_issuance_signer_address, state::IssuanceConfig,
    },
    openindex::{
        error::ProtocolError, instruction::ProtocolInstruction, pda::find_module_address,
        state::IndexMints,
    },
    require,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke,invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn issue(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    index_id: u64,
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let signer = next_account_info(accounts_iter)?;
    let issuance_config_account = next_account_info(accounts_iter)?;
    let issuance_signer_account = next_account_info(accounts_iter)?;
    let controller_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority_account = next_account_info(accounts_iter)?;
    let index_mints_account = next_account_info(accounts_iter)?;
    let module_account = next_account_info(accounts_iter)?;
    let openindex_program_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;

    require!(signer.is_signer, ProgramError::MissingRequiredSignature);

    require!(
        issuance_config_account.owner == program_id,
        IssuanceError::UnknownIssuanceConfigAccount.into()
    );

    let (issuance_signer_pda, issuance_signer_bump) = find_issuance_signer_address(program_id);

    require!(
        *issuance_signer_account.key == issuance_signer_pda,
        IssuanceError::IncorrectIssuanceSignerAccount.into()
    );

    let issuance_config = IssuanceConfig::try_from_slice(&issuance_config_account.data.borrow())?;

    for hook in issuance_config.allowed_hooks.iter() {
        let hook_account = next_account_info(accounts_iter)?;

        require!(
            *hook_account.key == *hook,
            IssuanceError::IncorrectHookAccount.into()
        );

        let instruction = ExecuteHookInstruction::Execute { index_id, amount };
        let mut instruction_data = vec![];
        instruction.serialize(&mut instruction_data)?;

        // invoke hook account
        invoke(
            &Instruction {
                program_id: *hook_account.key,
                accounts: vec![AccountMeta::new(*signer.key, true)],

                data: instruction_data,
            },
            &[signer.clone()],
        )?;
    }


    let (module_pda, module_bump) =
        find_module_address(openindex_program_account.key, &issuance_signer_pda);

    let index_mints_data = IndexMints::try_from_slice(&index_mints_account.data.borrow())?;

    require!(
        *module_account.key == module_pda,
        ProtocolError::InvalidModuleAccount.into()
    );

    let mut cpi_accounts = vec![
        AccountMeta::new_readonly(*issuance_signer_account.key, true),
        AccountMeta::new_readonly(*controller_account.key, false),
        AccountMeta::new(*mint_account.key, false),
        AccountMeta::new_readonly(*mint_authority_account.key, false),
        AccountMeta::new(*token_account.key, false),
        AccountMeta::new(*token_program_account.key, false),
        AccountMeta::new_readonly(*module_account.key, false),
    ];

    let mut cpi_account_infos = vec![
        issuance_signer_account.clone(),
        controller_account.clone(),
        mint_account.clone(),
        mint_authority_account.clone(),
        token_account.clone(),
        token_program_account.clone(),
        module_account.clone(),
    ];

    for (idx, mint) in index_mints_data.mints.iter().enumerate() {
        let component_mint_account = next_account_info(accounts_iter)?;
        let component_account = next_account_info(accounts_iter)?;
        let vault_pda = next_account_info(accounts_iter)?;
        let vault_ata = next_account_info(accounts_iter)?;
        let component_token_account = next_account_info(accounts_iter)?;

        cpi_accounts.push(AccountMeta::new_readonly(
            *component_mint_account.key,
            false,
        ));
        cpi_accounts.push(AccountMeta::new_readonly(*component_account.key, false));
        cpi_accounts.push(AccountMeta::new_readonly(*vault_pda.key, false));
        cpi_accounts.push(AccountMeta::new(*vault_ata.key, false));
        cpi_accounts.push(AccountMeta::new(*component_token_account.key, false));
        //
        cpi_account_infos.push(component_mint_account.clone());
        cpi_account_infos.push(component_account.clone());
        cpi_account_infos.push(vault_pda.clone());
        cpi_account_infos.push(vault_ata.clone());
        cpi_account_infos.push(component_token_account.clone());
    }

    let instruction = ProtocolInstruction::Mint { index_id, amount };
    let mut instruiction_data = vec![];
    instruction.serialize(&mut instruiction_data)?;

    invoke_signed(
        &Instruction {
            program_id: *openindex_program_account.key,
            accounts: cpi_accounts,
            data: instruiction_data,
        },
        &cpi_account_infos,
        &[&[program_id.as_ref(), &[issuance_signer_bump]]],
    )?;

    Ok(())
}
