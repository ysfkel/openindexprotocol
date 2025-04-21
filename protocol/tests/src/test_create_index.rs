use core::num;

use crate::{
    create_index_transaction, create_mint_acccount_transaction, get_controller_pda,
    get_protocol_pda, init_controller_transaction, init_protocol_transaction, setup, Setup,
};
use bincode::{config, decode_from_slice};
use borsh::{BorshDeserialize, BorshSerialize};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use open_index::{
    error::ProtocolError,
    state::{Controller, Protocol},
};
use open_index_lib::seeds::PROTOCOL_SEED;
use solana_program_test::BanksClientError;
use solana_sdk::{
    account::Account,
    clock::sysvar,
    instruction::InstructionError,
    msg,
    program_pack::Pack,
    rent::Rent,
    signature::Keypair,
    syscalls,
    system_instruction::create_account,
    sysvar::{Sysvar, SysvarId},
    transaction::TransactionError,
};
use spl_token::state::Mint;
use {solana_program::pubkey::Pubkey, solana_program_test::tokio, solana_sdk::signature::Signer};
#[tokio::test]
async fn test_create_index() {
    let _setup: Setup = setup().await;
    let mint = Keypair::new();
    let manager = Keypair::new();
    let mint_space = Mint::LEN;

    let lamports = _setup.rent.minimum_balance(mint_space);

    let create_account_instruction = create_mint_acccount_transaction(&mint, &_setup).await;

    let create_index_tx =
        create_index_transaction(1, 1, mint.pubkey(), manager.pubkey(), &_setup).await;

   let result = _setup.banks_client.process_transaction(create_account_instruction.transaction).await;

//    let result = _setup.banks_client.process_transaction(create_index_tx.transaction).await.err();//.err().unwrap();

// //    assert!(result);

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
