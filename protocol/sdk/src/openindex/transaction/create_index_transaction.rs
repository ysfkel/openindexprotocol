use crate::openindex::{
    instruction::create_index_instruction,
    pda::{
        find_controller_address, find_controller_global_config_address, find_index_address,
        find_index_mint_address,
    },
};
use solana_sdk::{hash::Hash, signature::Keypair, transaction::Transaction};
use {solana_program::pubkey::Pubkey, solana_sdk::signature::Signer};

/// Creates a transaction to create an index
pub fn create_index_transaction(
    payer: &Keypair,
    program_id: Pubkey,
    index_id: u64,
    controller_id: u64,
    manager: Pubkey,
    recent_blockhashes: Hash,
) -> Transaction {
    let controller_pda = find_controller_address(&program_id, controller_id).0;
    let index_pda = find_index_address(&program_id, &controller_pda, index_id).0;
    let (controller_global, _) = find_controller_global_config_address(&program_id);
    let mint = find_index_mint_address(&program_id, &controller_pda, index_id).0;
    let instruction = create_index_instruction(
        program_id.clone(),
        payer.pubkey().clone(),
        manager,
        index_pda,
        mint,
        controller_pda,
        controller_global,
    );

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhashes,
    )
}
