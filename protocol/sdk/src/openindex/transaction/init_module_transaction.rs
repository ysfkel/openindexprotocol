use crate::openindex::{
    instruction::init_module_instruction,
    pda::{find_module_signer_address, find_protocol_address, find_registered_module_address},
};
use solana_sdk::{hash::Hash, transaction::Transaction};

use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub fn init_module_transaction(
    payer: &Keypair,
    program_id: Pubkey,
    module_program_id: Pubkey,
    recent_blockhashes: Hash,
) -> Transaction {
    let protocol_pda = find_protocol_address(&program_id).0;
    let module_signer_pda = find_module_signer_address(&module_program_id).0;
    let registered_module_pda = find_registered_module_address(&program_id, &module_signer_pda).0;

    let instruction = init_module_instruction(
        program_id.clone(),
        payer.pubkey().clone(),
        protocol_pda,
        module_signer_pda,
        registered_module_pda,
    );

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhashes,
    )
}
