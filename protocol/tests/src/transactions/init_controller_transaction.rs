use crate::{find_controller_address, get_protocol_pda, Setup};
use borsh::BorshSerialize;
use open_index_lib::instruction::ProtocolInstruction;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    system_program,
    transaction::Transaction,
};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub struct InitControllerTransaction {
    pub controller_pda: Pubkey,
    pub transaction: Transaction,
}

pub fn init_controller_transaction(
    controller_id: u64,
    _setup: &Setup,
) -> InitControllerTransaction {
    let payer = &_setup.payer;
    let program_id = &_setup.program_id;
    let recent_blockhashes = &_setup.recent_blockhashes;
    let protocol_pda = get_protocol_pda(program_id).0;
    let controller_pda = find_controller_address(program_id, controller_id).0;
    
    let instruction = ProtocolInstruction::init_controller(program_id.clone(), payer.pubkey().clone(), protocol_pda.clone(), controller_pda.clone());
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        *recent_blockhashes,
    );
    InitControllerTransaction {
        transaction,
        controller_pda,
    }
}
