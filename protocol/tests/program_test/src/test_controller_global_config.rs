use crate::{
    process_controller_global_config, setup, BanksClientResult,
    ProcessControllerGlobalConfigResult, Setup,
};
use borsh::BorshDeserialize;
use openindex_sdk::openindex::{
    state::ControllerGlobalConfig,
    pda::find_controller_global_config_address,
    transaction::{init_controller_global_config_transaction, init_protocol_transaction},
};
use {solana_program_test::tokio, solana_sdk::signature::Signer};

#[tokio::test]
async fn test_controller_global_config() {
    let _setup: Setup = setup().await;
    let max_index_components = 10;
    let ProcessControllerGlobalConfigResult { result } =
        process_controller_global_config(max_index_components, &_setup).await;

    let controller_global_pda = find_controller_global_config_address(&_setup.program_id).0;
    let controller_global_account = _setup
        .banks_client
        .get_account(controller_global_pda)
        .await
        .unwrap()
        .unwrap();

    let cg = ControllerGlobalConfig::try_from_slice(&controller_global_account.data).unwrap();

    assert!(result.is_err() == false);
    assert_eq!(cg.initialized, true);
    assert_eq!(cg.max_index_components, max_index_components);
}
