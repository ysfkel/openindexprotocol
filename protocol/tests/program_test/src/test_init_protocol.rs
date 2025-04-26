use crate::{setup, Setup};
use borsh::BorshDeserialize;
use open_index::state::Protocol;
use open_index_lib::{pda::find_protocol_address, transaction::init_protocol_transaction};
use {solana_program_test::tokio, solana_sdk::signature::Signer};

#[tokio::test]
async fn test_init_protocol() {
    let _setup: Setup = setup().await;

    let transaction =
        init_protocol_transaction(&_setup.payer, _setup.program_id, _setup.recent_blockhashes);

    let result = _setup
        .banks_client
        .process_transaction(transaction.clone())
        .await;
    assert!(result.is_err() == false);

    let protocol_pda = find_protocol_address(&_setup.program_id).0;

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
