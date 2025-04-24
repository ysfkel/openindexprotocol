use crate::{init_module_transaction, setup, InitModuleTransaction, Setup};
use borsh::BorshDeserialize;
use open_index::state::Module;
use open_index_lib::transaction::init_protocol_transaction;
use solana_sdk::program_pack::IsInitialized;
use {solana_program_test::tokio, solana_sdk::signature::Signer};

#[tokio::test]
async fn test_init_module() {
    let _setup: Setup = setup().await;
    let max_index_components = 10;

    let init_protocol_instruction =
        init_protocol_transaction(&_setup.payer, _setup.program_id, _setup.recent_blockhashes);

    let _ = _setup
        .banks_client
        .process_transaction(init_protocol_instruction.clone())
        .await;

    let InitModuleTransaction {
        registered_module_pda,
        transaction,
    } = init_module_transaction(&_setup);

    let result = _setup
        .banks_client
        .process_transaction(transaction.clone())
        .await;

    assert!(result.is_err() == false);

    let registered_module_account = _setup
        .banks_client
        .get_account(registered_module_pda)
        .await
        .unwrap()
        .unwrap();

    let cg = Module::try_from_slice(&registered_module_account.data).unwrap();
    assert_eq!(cg.is_initialized(), true);
    assert_eq!(cg.is_active(), true);
}
