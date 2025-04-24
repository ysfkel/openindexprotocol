use core::num;

use crate::{setup, Setup};
use borsh::BorshDeserialize;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use open_index::{
    error::ProtocolError,
    state::{Controller, Protocol},
};

use open_index_lib::{
    pda::{find_controller_address, find_protocol_address},
    transaction::{init_controller_transaction, init_protocol_transaction},
};
use solana_program_test::BanksClientError;
use solana_sdk::{
    instruction::InstructionError, msg, program_pack::IsInitialized, transaction::TransactionError,
};
use {solana_program::pubkey::Pubkey, solana_program_test::tokio, solana_sdk::signature::Signer};

#[tokio::test]
async fn test_init_controller() {
    let _setup: Setup = setup().await;
    let program_id = _setup.program_id;
    let init_protocol_instruction =
        init_protocol_transaction(&_setup.payer, _setup.program_id, _setup.recent_blockhashes);

    // Send init_protocol_instruction
    let _ = _setup
        .banks_client
        .process_transaction(init_protocol_instruction.clone())
        .await;
    let protocol_pda = find_protocol_address(&program_id).0;

    // create controller
    let protocol_account = _setup
        .banks_client
        .get_account(protocol_pda)
        .await
        .unwrap()
        .unwrap();
    let protocol = Protocol::try_from_slice(&protocol_account.data).unwrap();
    let controller_id = protocol.get_next_controller_id();
    assert_eq!(protocol.next_controller_id, 1);
    let controller_pda = find_controller_address(&program_id, controller_id).0;
    let int_controller_tx = init_controller_transaction(
        &_setup.payer,
        _setup.program_id,
        controller_id,
        _setup.recent_blockhashes,
    );
    let _ = _setup
        .banks_client
        .process_transaction(int_controller_tx.clone())
        .await
        .err();

    let controller_account = _setup
        .banks_client
        .get_account(controller_pda)
        .await
        .unwrap()
        .unwrap();
    let protocol_account = _setup
        .banks_client
        .get_account(protocol_pda)
        .await
        .unwrap()
        .unwrap();
    let protocol = Protocol::try_from_slice(&protocol_account.data).unwrap();
    let controller = Controller::try_from_slice(&controller_account.data).unwrap();

    assert_eq!(controller.initialized, true);
    assert_eq!(protocol.next_controller_id, 2);
    assert_eq!(controller.owner, _setup.payer.pubkey());
    // create controller 2 - checks next_controller_id
    let protocol_account = _setup
        .banks_client
        .get_account(protocol_pda)
        .await
        .unwrap()
        .unwrap();
    let protocol = Protocol::try_from_slice(&protocol_account.data).unwrap();
    let controller_id = protocol.get_next_controller_id();
    assert_eq!(protocol.next_controller_id, 2);
    let controller_pda = find_controller_address(&program_id, controller_id).0;
    let int_controller_tx = init_controller_transaction(
        &_setup.payer,
        _setup.program_id,
        controller_id,
        _setup.recent_blockhashes,
    );
    let _: Option<BanksClientError> = _setup
        .banks_client
        .process_transaction(int_controller_tx.clone())
        .await
        .err();

    let controller_account = _setup
        .banks_client
        .get_account(controller_pda)
        .await
        .unwrap()
        .unwrap();
    let protocol_account = _setup
        .banks_client
        .get_account(protocol_pda)
        .await
        .unwrap()
        .unwrap();
    let protocol = Protocol::try_from_slice(&protocol_account.data).unwrap();
    let controller = Controller::try_from_slice(&controller_account.data).unwrap();
    assert_eq!(protocol.next_controller_id, 3);
    assert!(controller.is_initialized());
    assert_eq!(controller.get_next_index_id(), 1);
    // unwrap transaction error when ProtocolError is caught IncorrectControllerAccount
    // if let Some(error) = result {
    //     match error {
    //         BanksClientError::Io(e) => {
    //             println!("io error");
    //         }
    //         BanksClientError::RpcError(e) => {
    //             println!(" RpcError {:?}", e);
    //         }
    //         BanksClientError::TransactionError(tx_error) => {
    //             match tx_error {
    //                 TransactionError::InstructionError(n, ix_error) => {
    //                     match ix_error {
    //                         InstructionError::Custom(code) => {
    //                             match code {
    //                                 7 => {
    //                                     println!("hello controller roor");
    //                                     //  return Err(ProtocolError::IncorrectControllerAccount.into()); // Return error
    //                                 }

    //                                 _ => {}
    //                             }
    //                         }
    //                         _ => {}
    //                     }
    //                 }
    //                 _ => {}
    //             }
    //         }
    //         _ => {}
    //     }
    // }
}
