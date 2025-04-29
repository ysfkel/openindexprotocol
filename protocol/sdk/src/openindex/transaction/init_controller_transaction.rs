use crate::openindex::{
    instruction::init_controller_instruction,
    pda::{find_controller_address, find_protocol_address},
};
use solana_sdk::{
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    system_program,
    transaction::Transaction,
};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub fn init_controller_transaction(
    payer: &Keypair,
    program_id: Pubkey,
    controller_id: u64,
    recent_blockhashes: Hash,
) -> Transaction {
    let protocol_pda = find_protocol_address(&program_id).0;
    let controller_pda = find_controller_address(&program_id, controller_id).0;

    let instruction = init_controller_instruction(
        program_id.clone(),
        payer.pubkey().clone(),
        protocol_pda.clone(),
        controller_pda.clone(),
    );

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhashes,
    )
}
