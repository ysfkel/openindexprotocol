use crate::{get_protocol_pda, Setup};
use borsh::{BorshDeserialize, BorshSerialize};
use open_index_lib::{instruction::Instruction as OpenIndexInstruction, seeds::PROTOCOL_SEED};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    system_program,
    transaction::Transaction,
};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub struct InitProtocolTransaction {
    pub protocol_pda: Pubkey,
    pub transaction: Transaction,
}

pub async fn init_protocol_transaction(
    Setup {
        banks_client,
        recent_blockhashes,
        payer,
        program_id,
        rent,
    }: &Setup,
) -> InitProtocolTransaction {
    let (protocol_pda, _) = get_protocol_pda(program_id);
    let initialize_ix = &OpenIndexInstruction::InitProtocol;
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
                solana_sdk::instruction::AccountMeta::new_readonly(system_program::ID, false),
            ],
        )],
        Some(&payer.pubkey()),
        &[&payer],
        *recent_blockhashes,
    );
    InitProtocolTransaction {
        protocol_pda,
        transaction,
    }
}
