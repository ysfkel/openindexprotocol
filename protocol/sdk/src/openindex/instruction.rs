use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;
use spl_associated_token_account::get_associated_token_address_with_program_id;

use super::pda::find_component_address;
use super::pda::find_component_vault_address;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum ProtocolInstruction {
    // 0
    /// Initializes the Open Index Protocol account
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` Owner account. Can be governance account
    ///   1. `[writable]` Protocol account
    ///   2. `[]` System Program account
    InitProtocol,
    InitController,

    InitControllerGlobalConfig {
        max_index_components: u32,
    },
    InitModule,
    CreateIndex,
    AddIndexComponents {
        amounts: Vec<u64>,
        mints: Vec<Pubkey>,
    },
    Mint {
        index_id: u64,
        amount: u64,
    },
    Redeem,
    //..
}

pub fn init_protocol_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    protocol_account: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(caller, true),
        AccountMeta::new(protocol_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let instruction = ProtocolInstruction::InitProtocol;
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

pub fn init_controller_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    protocol_account: Pubkey,
    controller_account: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(caller, true),
        AccountMeta::new(protocol_account, false),
        AccountMeta::new(controller_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let instruction = ProtocolInstruction::InitController;
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

pub fn init_controller_global_config_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    protocol_account: Pubkey,
    controller_global_config_account: Pubkey,
    max_index_components: u32,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(caller, true),
        AccountMeta::new(protocol_account, false),
        AccountMeta::new(controller_global_config_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let instruction = ProtocolInstruction::InitControllerGlobalConfig {
        max_index_components,
    };
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_index_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    manager: Pubkey,
    index_account: Pubkey,
    mint_account: Pubkey,
    controller_account: Pubkey,
    controller_global_config_account: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(caller, true),
        AccountMeta::new(manager, false),
        AccountMeta::new(index_account, false),
        AccountMeta::new(mint_account, false),
        AccountMeta::new(controller_account, false),
        AccountMeta::new(controller_global_config_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(spl_token::ID, false),
    ];
    let instruction = ProtocolInstruction::CreateIndex;
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

pub fn init_module_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    protocol_account: Pubkey,
    module_signer_account: Pubkey,
    registered_module_account: Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(caller, true),
        AccountMeta::new(protocol_account, false),
        AccountMeta::new(module_signer_account, false),
        AccountMeta::new(registered_module_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let instruction = ProtocolInstruction::InitModule;
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn add_index_components_instruction(
    program_id: Pubkey,
    caller: Pubkey,
    index_account: Pubkey,
    index_mints_data_account: Pubkey,
    controller_account: Pubkey,
    controller_global_config_account: Pubkey,
    mints: Vec<Pubkey>,
    amounts: Vec<u64>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(caller, true),
        AccountMeta::new(index_account, false),
        AccountMeta::new(index_mints_data_account, false),
        AccountMeta::new(controller_account, false),
        AccountMeta::new(controller_global_config_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        AccountMeta::new_readonly(spl_token::ID, false),
    ];

    let instruction = ProtocolInstruction::AddIndexComponents { amounts, mints };
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn add_index_components_instruction_with_dynamic_accounts(
    program_id: Pubkey,
    caller: Pubkey,
    index_account: Pubkey,
    index_mints_data_account: Pubkey,
    controller_account: Pubkey,
    controller_global_config_account: Pubkey,
    mints: Vec<Pubkey>,
    amounts: Vec<u64>,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(caller, true),
        AccountMeta::new(index_account, false),
        AccountMeta::new(index_mints_data_account, false),
        AccountMeta::new(controller_account, false),
        AccountMeta::new(controller_global_config_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        AccountMeta::new_readonly(spl_token::ID, false),
    ];

    for mint in mints.iter() {
        let (component_pda, _) = find_component_address(&program_id, &index_account, mint);
        let (vault_pda, _) = find_component_vault_address(&program_id, &index_account, mint);
        let vault_ata =
            get_associated_token_address_with_program_id(&vault_pda, mint, &spl_token::ID);

        accounts.push(AccountMeta::new(mint.clone(), false));
        accounts.push(AccountMeta::new(component_pda, false));
        accounts.push(AccountMeta::new(vault_pda, false));
        accounts.push(AccountMeta::new(vault_ata, false));
    }

    let instruction = ProtocolInstruction::AddIndexComponents { amounts, mints };
    let data = borsh::to_vec(&instruction).unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}
