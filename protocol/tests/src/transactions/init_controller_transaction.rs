use crate::{get_controller_pda, get_protocol_pda, Setup};
use borsh::BorshSerialize;
use open_index_lib::instruction::Instruction as OpenIndexInstruction;
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

pub async fn init_controller_transaction(
    controller_id: u64,
    _setup: &Setup,
) -> InitControllerTransaction {
    let payer = &_setup.payer;
    let program_id = &_setup.program_id;
    let recent_blockhashes = &_setup.recent_blockhashes;
    let protocol_pda = get_protocol_pda(program_id).0;
    let controller_pda = get_controller_pda(program_id, controller_id).0;

    let initialize_ix = &OpenIndexInstruction::InitController;
    let mut initialize_ix_data = Vec::new();
    initialize_ix.serialize(&mut initialize_ix_data).unwrap();
    // use this for calling my program
    let transaction = Transaction::new_signed_with_payer(
        &[Instruction::new_with_borsh(
            program_id.clone(),
            &initialize_ix,
            vec![
                AccountMeta::new(payer.pubkey().clone(), true),
                AccountMeta::new(protocol_pda, false),
                AccountMeta::new(controller_pda, false),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
        )],
        Some(&payer.pubkey()),
        &[&payer],
        *recent_blockhashes,
    );
    InitControllerTransaction {
        transaction,
        controller_pda,
    }
}
