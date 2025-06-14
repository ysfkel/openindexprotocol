use crate::{
    process_controller_global_config, process_create_index, process_init_controller,
    process_init_protocol, setup, ProcessCreateIndexResult, ProcessInitControllerResult, Setup,
};

use borsh::BorshDeserialize;
use openindex_sdk::openindex::{
    state::{Controller, Index, Protocol},
    pda::{
        find_controller_address, find_index_address, find_index_mint_address, find_protocol_address,
    },
    transaction::{
        create_index_transaction, init_controller_global_config_transaction,
        init_controller_transaction, init_protocol_transaction,
    },
};
use solana_program_test::BanksClientError;
use solana_sdk::{
    clock::sysvar,
    instruction::InstructionError,
    msg,
    program_pack::{IsInitialized, Pack},
    rent::Rent,
    signature::Keypair,
    syscalls,
    system_instruction::create_account,
    sysvar::Sysvar,
    transaction::TransactionError,
};

use {solana_program_test::tokio, solana_sdk::signature::Signer};
#[tokio::test]
async fn test_create_index() {
    let _setup: Setup = setup().await;
    let program_id = _setup.program_id;
    let manager = Keypair::new();
    //Initialize Protocol
    let _ = process_init_protocol(&_setup).await;

    let protocol_pda = find_protocol_address(&program_id).0;

    let protocol_account = _setup
        .banks_client
        .get_account(protocol_pda)
        .await
        .unwrap()
        .unwrap();

    let _ = process_controller_global_config(10, &_setup).await;

    // Initialize Controller
    let ProcessInitControllerResult {
        controller_id,
        controller_pda,
        result,
    } = process_init_controller(&_setup).await;

    let ProcessCreateIndexResult {
        index_id,
        controller_pda,
        result,
    } = process_create_index(controller_id, manager.pubkey(), &_setup).await;

    let index_pda = find_index_address(&program_id, &controller_pda, 1).0;

    let index_account = _setup
        .banks_client
        .get_account(index_pda)
        .await
        .unwrap()
        .unwrap();

    let controller_account = _setup
        .banks_client
        .get_account(controller_pda)
        .await
        .unwrap()
        .unwrap();
    let controller = Controller::try_from_slice(&controller_account.data).unwrap();

    let index = Index::try_from_slice(&index_account.data).unwrap();
    assert!(index.is_initialized());
    assert_eq!(index.manager, manager.pubkey());
    assert_eq!(index.owner, _setup.payer.pubkey());
    assert!(!result.is_err());
    assert_eq!(controller.get_next_index_id(), 2);
}
