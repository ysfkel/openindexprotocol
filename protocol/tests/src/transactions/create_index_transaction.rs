use crate::Setup;
use borsh::BorshSerialize;
use open_index_lib::{
    instruction::{create_index_instruction, ProtocolInstruction},
    pda::{find_controller_address, find_controller_global_config_address, find_index_address},
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    system_program,
    transaction::Transaction,
};
use {solana_program::pubkey::Pubkey, solana_sdk::signature::Signer};

pub struct CreateIndexTransaction {
    pub index_pda: Pubkey,
    pub transaction: Transaction,
}

pub fn create_index_transaction(
    index_id: u64,
    controller_id: u64,
    mint: Pubkey,
    manager: Pubkey,
    _setup: &Setup,
) -> CreateIndexTransaction {
    let payer = &_setup.payer;
    let program_id = &_setup.program_id;
    let recent_blockhashes = &_setup.recent_blockhashes;
    let controller_pda = find_controller_address(program_id, controller_id).0;
    let (index_pda, _) = find_index_address(program_id, &controller_pda, index_id);
    let (controller_global, _) = find_controller_global_config_address(program_id);

    let instruction = create_index_instruction(
        program_id.clone(),
        payer.pubkey().clone(),
        manager,
        index_pda,
        mint,
        controller_pda,
        controller_global,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        *recent_blockhashes,
    );

    CreateIndexTransaction {
        index_pda,
        transaction,
    }
}
