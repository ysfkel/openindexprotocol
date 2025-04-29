use solana_program::{hash::Hash,instruction::{AccountMeta, Instruction}, pubkey::Pubkey};
use borsh::{BorshDeserialize, BorshSerialize};
use spl_associated_token_account::get_associated_token_address_with_program_id;

use crate::openindex::pda::{find_component_address, find_component_vault_address}; 

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum IssuanceInstruction {
    Mint { index_id: u64, amount: u64 },
    Redeem,
}

pub fn mint_index_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    module_account: Pubkey,
    registered_module_account: Pubkey,
    controller_account: Pubkey,
    mint_account: Pubkey,
    mint_authority_account: Pubkey,
    index_account: Pubkey,
    index_mints_data_account: Pubkey,
    open_index_account: Pubkey,
    token_account_account: Pubkey,
    token_program_account: Pubkey,

) -> Instruction {
    let accounts = vec![
        AccountMeta::new(caller, true),
        AccountMeta::new_readonly(module_account, false),
        AccountMeta::new_readonly(registered_module_account, false),
        AccountMeta::new_readonly(controller_account, false),
        AccountMeta::new(mint_account, false),
        AccountMeta::new_readonly(mint_authority_account, false),
        AccountMeta::new_readonly(index_account, false),
        AccountMeta::new_readonly(index_mints_data_account, false),
        AccountMeta::new_readonly(open_index_account, false),
        AccountMeta::new(token_account_account, false),
        AccountMeta::new(spl_token::ID, false),
    ];
    let instruction = ProtocolInstruction::InitProtocol;
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
  
}
 
#[allow(clippy::too_many_arguments)]
pub fn mint_index_instruction_with_dynamic_accounts(
    program_id: Pubkey,
    caller: Pubkey,
    module_account: Pubkey,
    registered_module_account: Pubkey,
    controller_account: Pubkey,
    mint_account: Pubkey,
    mint_authority_account: Pubkey,
    index_account: Pubkey,
    index_mints_data_account: Pubkey,
    open_index_account: Pubkey,
    token_account_account: Pubkey,
    token_program_account: Pubkey,
    mints: Vec<Pubkey>,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(caller, true),
        AccountMeta::new_readonly(module_account, false),
        AccountMeta::new_readonly(registered_module_account, false),
        AccountMeta::new_readonly(controller_account, false),
        AccountMeta::new(mint_account, false),
        AccountMeta::new_readonly(mint_authority_account, false),
        AccountMeta::new_readonly(index_account, false),
        AccountMeta::new_readonly(index_mints_data_account, false),
        AccountMeta::new_readonly(open_index_account, false),
        AccountMeta::new(token_account_account, false),
        AccountMeta::new(spl_token::ID, false),
    ];
    let instruction = ProtocolInstruction::InitProtocol;
    let data = borsh::to_vec(&instruction).unwrap();
 
 
    for mint in mints.iter() {
        let (component_pda, _) = find_component_address(&program_id, &index_account, mint);
        let vault_ata =
            get_associated_token_address_with_program_id(&vault_pda, mint, &spl_token::ID);

        accounts.push(AccountMeta::new(mint.clone(), false));
        accounts.push(AccountMeta::new(component_pda, false));
        accounts.push(AccountMeta::new(vault_ata, false));
    }
    
    Instruction {
        program_id,
        accounts,
        data,
    }
}
