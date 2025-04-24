use crate::Setup;
use borsh::BorshSerialize;
use open_index_lib::{
    instruction::{add_index_components_instruction, ProtocolInstruction},
    pda::{
        find_controller_address, find_controller_global_config_address, find_index_address,
        find_index_mints_data_address,
    },
};
use solana_sdk::{hash::Hash, instruction::Instruction, transaction::Transaction};

use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub fn add_index_components_transaction(
    payer: &Keypair,
    program_id: &Pubkey,
    index_id: u64,
    controller_id: u64,
    recent_blockhashes: Hash,
    mints: Vec<Pubkey>,
    amounts: Vec<u64>,
) -> Transaction {
    let controller_pda = find_controller_address(program_id, controller_id).0;
    let (index_pda, _) = find_index_address(program_id, &controller_pda, index_id);
    let (controller_global, _) = find_controller_global_config_address(program_id);
    let (index_mints_data_pda, _) =
        find_index_mints_data_address(program_id, &controller_pda, index_id);

    let instruction = add_index_components_instruction(
        program_id.clone(),
        payer.pubkey().clone(),
        index_pda,
        index_mints_data_pda,
        controller_pda,
        controller_global,
        mints,
        amounts,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhashes,
    );

    transaction
}
