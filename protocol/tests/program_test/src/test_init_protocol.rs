use crate::{process_init_protocol, setup, ProcessInitProtocolResult, Setup};
use borsh::BorshDeserialize;
use openindex_sdk::openindex::{
    state::Protocol,
    pda::find_protocol_address, transaction::init_protocol_transaction,
};
use {solana_program_test::tokio, solana_sdk::signature::Signer};

#[tokio::test]
async fn test_init_protocol() {
    let _setup: Setup = setup().await;

    let ProcessInitProtocolResult { result } = process_init_protocol(&_setup).await;

    let protocol_pda = find_protocol_address(&_setup.program_id).0;

    let protocol_account = _setup
        .banks_client
        .get_account(protocol_pda)
        .await
        .unwrap()
        .unwrap();
    let protocol = Protocol::try_from_slice(&protocol_account.data).unwrap();
    assert!(result.is_err() == false);
    assert_eq!(protocol.initialized, true);
    assert_eq!(protocol.next_controller_id, 1);
    assert_eq!(protocol.owner, _setup.payer.pubkey());
}
