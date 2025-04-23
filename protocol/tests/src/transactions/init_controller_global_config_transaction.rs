use crate::{get_controller_global_config_pda, get_protocol_pda, Setup};
use borsh::BorshSerialize;
use open_index_lib::instruction::ProtocolInstruction;
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
    let protocol_pda = get_protocol_pda(program_id).0;
    let controller_global_pda = get_controller_global_config_pda(program_id).0;

     let instruction = ProtocolInstruction::init_controller_global_config(program_id.clone(), payer.pubkey().clone(), protocol_pda.clone(), controller_global_pda.clone(), max_index_components);

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
