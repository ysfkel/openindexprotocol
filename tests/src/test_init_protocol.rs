use borsh::{BorshDeserialize, BorshSerialize};
use open_index::state::Protocol;
use solana_program::example_mocks::solana_sdk::{system_instruction, sysvar::recent_blockhashes};
use solana_program_test::tokio::io::unix::TryIoError;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    rent::Rent,
    serde_varint::serialize,
    system_program,
    sysvar::Sysvar,
    transaction::Transaction,
};
use std::str::FromStr;

use {
    open_index::{
        entrypoint::process_instruction, instruction::Instruction as OpenIndexInstruction,
        seeds::PROTOCOL_SEED, state::Controller,
    },
    solana_program::pubkey::Pubkey,
    solana_program_test::{processor, tokio, ProgramTest},
    solana_sdk::signature::{Keypair, Signer},
    spl_token::state::Account as TokenAccount,
};

#[tokio::test]
async fn test_controller() {
    let program_id = Pubkey::new_unique();
    let (protocol_account, _) = Pubkey::find_program_address(&[PROTOCOL_SEED], &program_id);

    let program_test = ProgramTest::new("open_index", program_id, processor!(process_instruction));

    let (mut banks_client, payer, recent_blockhashes) = program_test.start().await;

    let initialize_ix = &OpenIndexInstruction::InitProtocol;
    let mut initialize_ix_data = Vec::new();
    initialize_ix.serialize(&mut initialize_ix_data).unwrap();

    //     // use this for calling my program
    let transaction = Transaction::new_signed_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &initialize_ix,
            vec![
                solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
                solana_sdk::instruction::AccountMeta::new(protocol_account, false),
                solana_sdk::instruction::AccountMeta::new_readonly(system_program::ID, false),
            ],
        )],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhashes,
    );
    let result = banks_client.process_transaction(transaction.clone()).await;
    // assert_eq!(result.is_err(), false);

    let sim_result = banks_client
        .simulate_transaction(transaction.clone())
        .await
        .unwrap();

    if let Some(details) = sim_result.simulation_details.as_ref() {
        for log in details.logs.iter() {
            println!("my loag {}", log);
        }
    }
}
