use core::num;

use crate::{
    get_controller_pda, get_protocol_pda, init_controller_transaction, init_protocol_instruction,
    setup, Setup,
};
use borsh::{BorshDeserialize, BorshSerialize};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use open_index::{
    error::ProtocolError,
    state::{Controller, Protocol},
};
use open_index_lib::seeds::PROTOCOL_SEED;
use solana_program_test::BanksClientError;
use solana_sdk::{instruction::InstructionError, msg, transaction::TransactionError};
use {solana_program::pubkey::Pubkey, solana_program_test::tokio, solana_sdk::signature::Signer};

#[tokio::test]
async fn test_init_controller() {
    let _setup: Setup = setup().await;
    let program_id = _setup.program_id;
    let init_protocol_instruction = init_protocol_instruction(&_setup).await;
    let result = _setup
        .banks_client
        .process_transaction(init_protocol_instruction.transaction.clone())
        .await;
    let protocol_pda = get_protocol_pda(&program_id).0;
    let protocol_account = _setup
        .banks_client
        .get_account(protocol_pda)
        .await
        .unwrap()
        .unwrap();
    let mut protocol = Protocol::try_from_slice(&protocol_account.data).unwrap();

    // create controller
    let controller_id = protocol.get_next_controller_id();
    let controller_pda = get_controller_pda(&program_id, controller_id).0;
    let int_controller_tx = init_controller_transaction(controller_id, &_setup).await;
    let result: Option<BanksClientError> = _setup
        .banks_client
        .process_transaction(int_controller_tx.transaction.clone())
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
    let mut protocol = Protocol::try_from_slice(&protocol_account.data).unwrap();
    let controller = Controller::try_from_slice(&controller_account.data).unwrap();
    assert_eq!(controller.initialized, true);
    assert_eq!(protocol.next_controller_id, 2);
    assert_eq!(controller.owner, _setup.payer.pubkey());

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
