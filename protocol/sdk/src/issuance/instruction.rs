use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use spl_associated_token_account::get_associated_token_address_with_program_id;

use crate::openindex::pda::{find_component_address, find_component_vault_address};

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum IssuanceInstruction {
    Mint { index_id: u64, amount: u64 },
    Redeem,
}

pub fn mint_index_instruction(
    caller: Pubkey,
    issuance_program_id: Pubkey,
    open_index_program_id: Pubkey,
    module_signer_account: Pubkey,
    registered_module_account: Pubkey,
    controller_account: Pubkey,
    mint_account: Pubkey,
    //   mint_authority_account: Pubkey,
    index_account: Pubkey,
    index_mints_data_account: Pubkey,
    token_account: Pubkey,
    token_program_account: Pubkey,
    index_id: u64,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(caller, true),
        AccountMeta::new_readonly(module_signer_account, false),
        AccountMeta::new_readonly(registered_module_account, false),
        AccountMeta::new_readonly(controller_account, false),
        AccountMeta::new(mint_account, false),
        //   AccountMeta::new_readonly(mint_authority_account, false),
        AccountMeta::new_readonly(index_account, false),
        AccountMeta::new_readonly(index_mints_data_account, false),
        AccountMeta::new_readonly(open_index_program_id, false),
        AccountMeta::new(token_account, false),
        AccountMeta::new(token_program_account, false),
    ];
    let instruction = IssuanceInstruction::Mint { index_id, amount };
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id: issuance_program_id,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn mint_index_instruction_with_dynamic_accounts(
    caller: Pubkey,
    issuance_program_id: Pubkey,
    open_index_program_id: Pubkey,
    module_signer_account: Pubkey,
    registered_module_account: Pubkey,
    controller_account: Pubkey,
    mint_account: Pubkey,
    mint_authority_account: Pubkey,
    index_account: Pubkey,
    index_mints_data_account: Pubkey,
    token_account: Pubkey,
    token_program_account: Pubkey,
    mints: Vec<Pubkey>,
    token_accounts: Vec<Pubkey>,
    index_id: u64,
    amount: u64,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(caller, true),
        AccountMeta::new_readonly(module_signer_account, false),
        AccountMeta::new_readonly(registered_module_account, false),
        AccountMeta::new_readonly(controller_account, false),
        AccountMeta::new(mint_account, false),
        AccountMeta::new_readonly(mint_authority_account, false),
        AccountMeta::new_readonly(index_account, false),
        AccountMeta::new_readonly(index_mints_data_account, false),
        AccountMeta::new_readonly(open_index_program_id, false),
        AccountMeta::new(token_account, false),
        AccountMeta::new(token_program_account, false),
    ];
    let instruction = IssuanceInstruction::Mint { index_id, amount };
    let data = borsh::to_vec(&instruction).unwrap();

    for (index, _mint) in mints.iter().enumerate() {
        let (component_pda, _) =
            find_component_address(&open_index_program_id, &index_account, _mint);
        let (vault_pda, _) =
            find_component_vault_address(&open_index_program_id, &index_account, _mint);
        let vault_ata =
            get_associated_token_address_with_program_id(&vault_pda, _mint, &spl_token::ID);

        accounts.push(AccountMeta::new(_mint.clone(), false));
        accounts.push(AccountMeta::new(component_pda, false));
        accounts.push(AccountMeta::new(vault_pda, false));
        accounts.push(AccountMeta::new(vault_ata, false));
        let _token_account = token_accounts.get(index).unwrap();
        accounts.push(AccountMeta::new(*_token_account, false));
    }

    Instruction {
        program_id: issuance_program_id,
        accounts,
        data,
    }
}
