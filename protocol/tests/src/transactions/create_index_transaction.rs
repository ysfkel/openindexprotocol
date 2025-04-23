use crate::{find_controller_address, find_index_address, get_controller_global_config_pda, Setup};
use borsh::BorshSerialize;
use open_index_lib::instruction::ProtocolInstruction;
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
    let (controller_global, _) = get_controller_global_config_pda(program_id);

    let instruction = ProtocolInstruction::create_index(program_id.clone(), payer.pubkey().clone(), manager, index_pda, mint, controller_pda, controller_global);

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
