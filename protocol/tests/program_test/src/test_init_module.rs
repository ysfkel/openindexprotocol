use crate::{setup, Setup};
use borsh::BorshDeserialize;
use openindex::state::Module;
use openindex_sdk::openindex::{
    pda::{find_module_signer_address, find_registered_module_address},
    transaction::{init_module_transaction, init_protocol_transaction},
};
use solana_program::example_mocks::solana_keypair::Keypair;
use solana_sdk::{program_pack::IsInitialized, pubkey::Pubkey};
use {solana_program_test::tokio, solana_sdk::signature::Signer};

#[tokio::test]
async fn test_init_module() {
    let _setup: Setup = setup().await;
    let max_index_components = 10;
    let module_program_id = Pubkey::new_unique();

    let init_protocol_instruction =
        init_protocol_transaction(&_setup.payer, _setup.program_id, _setup.recent_blockhashes);

    let _ = _setup
        .banks_client
        .process_transaction(init_protocol_instruction.clone())
        .await;

    let transaction = init_module_transaction(
        &_setup.payer,
        _setup.program_id,
        module_program_id,
        _setup.recent_blockhashes,
    );

    let result = _setup
        .banks_client
        .process_transaction(transaction.clone())
        .await;

    assert!(result.is_err() == false);

    let module_signer_pda = find_module_signer_address(&module_program_id).0;
    let registered_module_pda =
        find_registered_module_address(&_setup.program_id, &module_signer_pda).0;

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
