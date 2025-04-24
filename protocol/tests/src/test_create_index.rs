use crate::{
    create_index_transaction, init_controller_global_config_transaction,
    init_controller_transaction, init_protocol_transaction, setup, Setup,
};

use borsh::BorshDeserialize;
use open_index::state::{Controller, Index, Protocol};
use open_index_lib::pda::{find_index_mint_address, find_protocol_address};
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
    let init_protocol_instruction = init_protocol_transaction(&_setup);
    let protocol_pda = find_protocol_address(&program_id).0;

    let _ = _setup
        .banks_client
        .process_transaction(init_protocol_instruction.transaction.clone())
        .await;

    let protocol_account = _setup
        .banks_client
        .get_account(protocol_pda)
        .await
        .unwrap()
        .unwrap();

    let protocol = Protocol::try_from_slice(&protocol_account.data).unwrap();
    // Initialize Controller
    let controller_id = protocol.get_next_controller_id();
    let init_controller_tx = init_controller_transaction(controller_id, &_setup);
    let controller_pda = init_controller_tx.controller_pda;
    let _ = _setup
        .banks_client
        .process_transaction(init_controller_tx.transaction.clone())
        .await
        .err();
    let controller_account = _setup
        .banks_client
        .get_account(controller_pda)
        .await
        .unwrap()
        .unwrap();
    let controller = Controller::try_from_slice(&controller_account.data).unwrap();
    // Create Index tx
    let mint =
        find_index_mint_address(&program_id, &controller_pda, controller.get_next_index_id()).0;
    let create_index_tx =
        create_index_transaction(1, controller.id, mint.clone(), manager.pubkey(), &_setup);
    // Create controller global  config tx
    let controller_global_tx = init_controller_global_config_transaction(10, &_setup);
    let _ = _setup
        .banks_client
        .process_transaction(controller_global_tx.transaction.clone())
        .await;
    //
    let _ = _setup
        .banks_client
        .process_transaction(init_controller_tx.transaction.clone())
        .await
        .err();
    let result = _setup
        .banks_client
        .process_transaction(create_index_tx.transaction)
        .await;

    let index_account = _setup
        .banks_client
        .get_account(create_index_tx.index_pda)
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

    //        if let Some(error) = result {
    //         match error {
    //             BanksClientError::Io(e) => {
    //                 println!("io error");
    //             }
    //             BanksClientError::RpcError(e) => {
    //                 println!(" RpcError {:?}", e);
    //             }
    //             BanksClientError::TransactionError(tx_error) => {
    //                 match tx_error {
    //                     TransactionError::InstructionError(n, ix_error) => {
    //                         match ix_error {
    //                             InstructionError::Custom(code) => {
    //                                 match code {
    //                                     7 => {
    //                                         println!("hello controller roor");
    //                                         //  return Err(ProtocolError::IncorrectControllerAccount.into()); // Return error
    //                                     }

    //                                     _ => {}
    //                                 }
    //                             }
    //                             _ => {}
    //                         }
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
}
