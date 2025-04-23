use crate::{
    init_controller_global_config_transaction, init_protocol_transaction, setup,
    InitControllerGlobalTransaction, Setup,
};
use borsh::BorshDeserialize;
use open_index::state::ControllerGlobalConfig;
use {solana_program_test::tokio, solana_sdk::signature::Signer};

#[tokio::test]
async fn test_controller_global_config() {
    let _setup: Setup = setup().await;
    let max_index_components = 10;
    let InitControllerGlobalTransaction {
        controller_global_pda,
        transaction,
    } = init_controller_global_config_transaction(max_index_components, &_setup);

    let init_protocol_instruction = init_protocol_transaction(&_setup);

    let _ = _setup
        .banks_client
        .process_transaction(init_protocol_instruction.transaction.clone())
        .await;

    let result = _setup
        .banks_client
        .process_transaction(transaction.clone())
        .await;
    assert!(result.is_err() == false);

    let controller_global_account = _setup
        .banks_client
        .get_account(controller_global_pda)
        .await
        .unwrap()
        .unwrap();

    let cg = ControllerGlobalConfig::try_from_slice(&controller_global_account.data).unwrap();
    assert_eq!(cg.initialized, true);
    assert_eq!(cg.max_index_components, max_index_components);
}
