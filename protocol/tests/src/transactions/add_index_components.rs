use crate::{
    find_component_address, find_component_vault_address, find_index_mints_data_address, get_controller_global_config_pda, get_controller_pda, get_index_pda, Setup
};
use borsh::BorshSerialize;
use open_index_lib::instruction::Instruction as OpenIndexInstruction;
use solana_sdk::{
     instruction::{AccountMeta, Instruction}, system_program, transaction::Transaction
};
use spl_associated_token_account::{get_associated_token_address, get_associated_token_address_with_program_id};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub struct AddIndexComponentsTransaction {
    pub index_mints_data_pda: Pubkey,
    pub components: Vec<Pubkey>,
    pub transaction: Transaction,
}

pub async fn add_index_components(
    index_id: u64,
    controller_id: u64,
    mints: Vec<Pubkey>, 
    amounts: Vec<u64>, 
    _setup: &Setup,
) -> AddIndexComponentsTransaction {
    let recent_blockhashes = &_setup.recent_blockhashes;
    let payer = &_setup.payer;
    let program_id = &_setup.program_id;
    let controller_pda = get_controller_pda(program_id, controller_id).0;
    let (index_pda, _) = get_index_pda(program_id, &controller_pda, index_id);
    let (controller_global, _) = get_controller_global_config_pda(program_id);
    let (index_mints_data_pda, _) = find_index_mints_data_address(program_id, &controller_pda, index_id); 
    let mut accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(index_pda, false),
        AccountMeta::new(index_mints_data_pda, false),
        AccountMeta::new(controller_pda, false),
        AccountMeta::new(controller_global, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        AccountMeta::new_readonly(spl_token::ID, false),
    ];

    let mut components = vec![];

    for mint in mints.iter() {
        let (component_pda, _) = find_component_address(program_id, &index_pda, mint);
        let (vault_pda, _) = find_component_vault_address(program_id, &index_pda, mint);
        let vault_ata = get_associated_token_address_with_program_id(&vault_pda, mint, &spl_token::ID);

        accounts.push(AccountMeta::new(mint.clone(),false));
        accounts.push(AccountMeta::new(component_pda, false));
        accounts.push(AccountMeta::new(vault_pda, false));
        accounts.push(AccountMeta::new(vault_ata, false));
        components.push(component_pda);
    }

    let initialize_ix = &OpenIndexInstruction::AddIndexComponents { amounts, mints };
    let mut initialize_ix_data = Vec::new();
    initialize_ix.serialize(&mut initialize_ix_data).unwrap();
    // use this for calling my program
    let transaction = Transaction::new_signed_with_payer(
        &[Instruction::new_with_borsh(
            program_id.clone(),
            &initialize_ix,
            accounts,
        )],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhashes.clone(),
    );

    AddIndexComponentsTransaction {
        index_mints_data_pda,
        components,
        transaction,
    }
}
