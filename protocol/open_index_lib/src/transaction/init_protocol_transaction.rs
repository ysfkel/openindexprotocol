use crate::{instruction::init_protocol_instruction, pda::find_protocol_address};
use solana_sdk::{
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    transaction::Transaction,
};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub fn init_protocol_transaction(
    payer: &Keypair,
    program_id: Pubkey,
    recent_blockhashes: Hash,
) -> Transaction {
    let (protocol_pda, _) = find_protocol_address(&program_id);

    let instruction = init_protocol_instruction(
        program_id.clone(),
        payer.pubkey().clone(),
        protocol_pda.clone(),
    );

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhashes,
    )
}
