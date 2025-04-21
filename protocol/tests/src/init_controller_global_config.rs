use crate::{get_controller_global_config_pda, get_controller_pda, get_protocol_pda, Setup};
use borsh::{BorshDeserialize, BorshSerialize};
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

pub struct InitControllerGlobalTransaction {
    pub controller_global_pda: Pubkey,
    pub transaction: Transaction,
}

pub async fn init_controller_global_config(
    max_index_components: u32,
    Setup {
        banks_client,
        recent_blockhashes,
        payer,
        program_id,
        rent,
    }: &Setup,
) -> InitControllerGlobalTransaction {
    let protocol_pda = get_protocol_pda(program_id).0;
    let controller_global_pda = get_controller_global_config_pda(program_id).0;

    let initialize_ix = &OpenIndexInstruction::InitControllerGlobalConfig { max_index_components };
    let mut initialize_ix_data = Vec::new();
    initialize_ix.serialize(&mut initialize_ix_data).unwrap();
    // use this for calling my program
    let transaction = Transaction::new_signed_with_payer(
        &[Instruction::new_with_borsh(
            program_id.clone(),
            &initialize_ix,
            vec![
                solana_sdk::instruction::AccountMeta::new(payer.pubkey().clone(), true),
                solana_sdk::instruction::AccountMeta::new(protocol_pda, false),
                solana_sdk::instruction::AccountMeta::new(controller_global_pda, false),
                solana_sdk::instruction::AccountMeta::new_readonly(system_program::ID, false),
            ],
        )],
        Some(&payer.pubkey()),
        &[&payer],
        *recent_blockhashes,
    );
    InitControllerGlobalTransaction {
        transaction,
        controller_global_pda,
    }
}
