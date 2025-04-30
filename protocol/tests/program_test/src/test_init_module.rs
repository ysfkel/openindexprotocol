use crate::{
    process_init_module, process_init_protocol, setup, BanksClientResult, ProcessInitModuleResult,
    Setup,
};
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

    let _ = process_init_protocol(&_setup).await;

    let ProcessInitModuleResult {
        registered_module_pda,
        module_signer_pda,
        result,
    } = process_init_module(_setup.issuance_program_id, &_setup).await;

    let registered_module_account = _setup
        .banks_client
        .get_account(registered_module_pda)
        .await
        .unwrap()
        .unwrap();

    assert!(result.is_err() == false);
    let cg = Module::try_from_slice(&registered_module_account.data).unwrap();
    assert_eq!(cg.is_initialized(), true);
    assert_eq!(cg.is_active(), true);
}
