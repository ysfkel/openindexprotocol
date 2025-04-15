use crate::{init_protocol_instruction, setup, InitProtocolTransaction, Setup};
use borsh::{BorshDeserialize, BorshSerialize};
use open_index::state::Protocol;
use {solana_program::pubkey::Pubkey, solana_program_test::tokio, solana_sdk::signature::Signer};

#[tokio::test]
async fn test_init_protocol() {
    let _setup: Setup = setup().await;
    let InitProtocolTransaction {
        protocol_pda,
        transaction,
    } = init_protocol_instruction(&_setup).await;
    let result = _setup
        .banks_client
        .process_transaction(transaction.clone())
        .await;
    let protocol_account = _setup
        .banks_client
        .get_account(protocol_pda)
        .await
        .unwrap()
        .unwrap();
    let protocol = Protocol::try_from_slice(&protocol_account.data).unwrap();
    assert_eq!(protocol.initialized, true);
    assert_eq!(protocol.next_controller_id, 1);
    assert_eq!(protocol.owner, _setup.payer.pubkey());
}
