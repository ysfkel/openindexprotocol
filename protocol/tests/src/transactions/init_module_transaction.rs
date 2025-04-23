use crate::{find_module_signer_address, find_registered_module_address, get_protocol_pda, Setup};
use borsh::{BorshDeserialize, BorshSerialize};
use open_index_lib::instruction::ProtocolInstruction;
use solana_sdk::{
    instruction::Instruction,
    system_program,
    transaction::Transaction,
};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub struct InitModuleTransaction {
    pub registered_module_pda: Pubkey,
    pub transaction: Transaction,
}

pub fn init_module_transaction(_setup: &Setup) -> InitModuleTransaction {
    let fake_module_program_id = Keypair::new().pubkey();
    let payer = &_setup.payer;
    let program_id = &_setup.program_id;
    let recent_blockhashes = &_setup.recent_blockhashes;
    let protocol_pda= get_protocol_pda(program_id).0;
    let module_signer_pda = find_module_signer_address(&fake_module_program_id).0;
    let registered_module_pda = find_registered_module_address(program_id, &module_signer_pda).0;

    let instruction = ProtocolInstruction::init_module(program_id.clone(),payer.pubkey().clone(), protocol_pda, module_signer_pda, registered_module_pda);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        *recent_blockhashes,
    );
    InitModuleTransaction {
        registered_module_pda,
        transaction,
    }
}
