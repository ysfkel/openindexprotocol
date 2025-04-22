use crate::{get_controller_global_config_pda, get_controller_pda, get_index_pda, Setup};
use borsh::BorshSerialize;
use open_index_lib::instruction::Instruction as OpenIndexInstruction;
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

pub async fn create_index_transaction(
    index_id: u64,
    controller_id: u64,
    mint: Pubkey,
    manager: Pubkey,
    _setup: &Setup,
) -> CreateIndexTransaction {
    let payer = &_setup.payer;
    let program_id = &_setup.program_id;
    let recent_blockhashes = &_setup.recent_blockhashes;
    let controller_pda = get_controller_pda(program_id, controller_id).0;
    let (index_pda, _) = get_index_pda(program_id, &controller_pda, index_id);
    let (controller_global, _) = get_controller_global_config_pda(program_id);

    let initialize_ix = &OpenIndexInstruction::CreateIndex;
    let mut initialize_ix_data = Vec::new();
    initialize_ix.serialize(&mut initialize_ix_data).unwrap();
    // use this for calling my program
    let transaction = Transaction::new_signed_with_payer(
        &[Instruction::new_with_borsh(
            program_id.clone(),
            &initialize_ix,
            vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(manager, false),
                AccountMeta::new(index_pda, false),
                AccountMeta::new(mint, false),
                AccountMeta::new(controller_pda, false),
                AccountMeta::new(controller_global, false),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new_readonly(spl_token::ID, false),
            ],
        )],
        Some(&payer.pubkey()),
        &[&payer],
        *recent_blockhashes,
    );

    CreateIndexTransaction {
        index_pda,
        transaction,
    }
}
