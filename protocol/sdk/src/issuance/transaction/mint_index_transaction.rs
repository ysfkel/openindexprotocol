use crate::{issuance::instruction::mint_index_instruction_with_dynamic_accounts, openindex::{
    instruction::add_index_components_instruction_with_dynamic_accounts,
    pda::{
        find_component_address, find_component_vault_address, find_controller_address,
        find_controller_global_config_address, find_index_address, find_index_mints_data_address,
    },
}};
use solana_sdk::{
    hash::Hash,
    instruction::Instruction,
    transaction::{Transaction, VersionedTransaction},
};
use spl_associated_token_account::get_associated_token_address_with_program_id;

use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

/*
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
// */
// pub fn add_index_components_transaction(
//     payer: &Keypair,
//     program_id: Pubkey,
//     module_program_id: Pubkey,
//     index_id: u64,
//     controller_id: u64,
//     recent_blockhashes: Hash,
//     mints: Vec<Pubkey>,
//     amounts: Vec<u64>,
// ) -> Transaction {
//     let controller_pda = find_controller_address(&program_id, controller_id).0;
//     let index_pda = find_index_address(&program_id, &controller_pda, index_id).0;
//     let module_signer_pda = find_module_signer_address(&module_program_id).0;
//     let registered_module_pda = find_registered_module_address(&program_id, &module_signer_pda).0;

//     let (index_mints_data_pda, _) =
//         find_index_mints_data_address(&program_id, &controller_pda, index_id);

//     let instruction = mint_index_instruction_with_dynamic_accounts(
//         program_id.clone(),
//         payer.pubkey(),
//         module_signer_pda,
//         registered_module_pda,
//         controller_pda,
         
//     );

//     Transaction::new_signed_with_payer(
//         &[instruction],
//         Some(&payer.pubkey()),
//         &[&payer],
//         recent_blockhashes,
//     )
// }
