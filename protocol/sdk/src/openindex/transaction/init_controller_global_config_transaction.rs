use crate::openindex::{
    instruction::init_controller_global_config_instruction,
    pda::{find_controller_global_config_address, find_protocol_address},
};
use solana_sdk::{hash::Hash, signature::Keypair, transaction::Transaction};

use {solana_program::pubkey::Pubkey, solana_sdk::signature::Signer};

pub fn init_controller_global_config_transaction(
    payer: &Keypair,
    program_id: Pubkey,
    max_index_components: u32,
    recent_blockhashes: Hash,
) -> Transaction {
    let protocol_pda = find_protocol_address(&program_id).0;
    let controller_global_pda = find_controller_global_config_address(&program_id).0;

    let instruction = init_controller_global_config_instruction(
        program_id.clone(),
        payer.pubkey().clone(),
        protocol_pda.clone(),
        controller_global_pda.clone(),
        max_index_components,
    );

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhashes,
    )
}
