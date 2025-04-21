use crate::{
    get_controller_global_config_pda, get_controller_pda, get_index_pda, get_protocol_pda, Setup,
};
use borsh::{BorshDeserialize, BorshSerialize};
use open_index_lib::{
    instruction::Instruction as OpenIndexInstruction,
    seeds::{CONTROLLER_SEED, PROTOCOL_SEED},
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    system_program,
    transaction::Transaction,
};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub struct CreateIndexTransaction {
    index_pda: Pubkey,
    pub transaction: Transaction,
}

pub async fn create_index_transaction(
    index_id: u64,
    controller_id: u64,
    mint: Pubkey,
    manager: Pubkey,
    Setup {
        banks_client,
        recent_blockhashes,
        payer,
        program_id,
        rent,
    }: &Setup,
) -> CreateIndexTransaction{
    let controller_pda = get_controller_pda(program_id, controller_id).0;
    let (index_pda, _) = get_index_pda(program_id, controller_id, index_id);
    let (controller_global, _) = get_controller_global_config_pda(program_id);

    let initialize_ix = &OpenIndexInstruction::CreateIndex ;
    let mut initialize_ix_data = Vec::new();
    initialize_ix.serialize(&mut initialize_ix_data).unwrap();
    // use this for calling my program
    let transaction = Transaction::new_signed_with_payer(
        &[Instruction::new_with_borsh(
            program_id.clone(),
            &initialize_ix,
            vec![
                solana_sdk::instruction::AccountMeta::new(payer.pubkey().clone(), true),
                solana_sdk::instruction::AccountMeta::new(manager, false),
                solana_sdk::instruction::AccountMeta::new(index_pda, false),
                solana_sdk::instruction::AccountMeta::new(mint, false),
                solana_sdk::instruction::AccountMeta::new(controller_pda, false),
                solana_sdk::instruction::AccountMeta::new(controller_global, false),
                solana_sdk::instruction::AccountMeta::new_readonly(system_program::ID, false),
                solana_sdk::instruction::AccountMeta::new_readonly(spl_token::ID, false),
            ],
        )],
        Some(&payer.pubkey()),
        &[&payer],
        *recent_blockhashes,
    );

    CreateIndexTransaction {
        index_pda,
        transaction
    }
}
