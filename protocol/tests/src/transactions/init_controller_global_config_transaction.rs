use crate::Setup;
use borsh::BorshSerialize;
use open_index_lib::{
    instruction::{init_controller_global_config_instruction, ProtocolInstruction},
    pda::{find_controller_global_config_address, find_protocol_address},
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    system_program,
    transaction::Transaction,
};
use {solana_program::pubkey::Pubkey, solana_sdk::signature::Signer};

pub struct InitControllerGlobalTransaction {
    pub controller_global_pda: Pubkey,
    pub transaction: Transaction,
}

pub fn init_controller_global_config_transaction(
    max_index_components: u32,
    _setup: &Setup,
) -> InitControllerGlobalTransaction {
    let payer = &_setup.payer;
    let recent_blockhashes = &_setup.recent_blockhashes;
    let program_id = &_setup.program_id;
    let protocol_pda = find_protocol_address(program_id).0;
    let controller_global_pda = find_controller_global_config_address(program_id).0;

    let instruction = init_controller_global_config_instruction(
        program_id.clone(),
        payer.pubkey().clone(),
        protocol_pda.clone(),
        controller_global_pda.clone(),
        max_index_components,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        *recent_blockhashes,
    );
    InitControllerGlobalTransaction {
        transaction,
        controller_global_pda,
    }
}
