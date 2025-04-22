use core::num;

use crate::{
    add_index_components_transaction, create_index_transaction, create_mint_acccount_transaction,
    get_index_mint_pda, init_controller_global_config, init_controller_transaction,
    init_protocol_transaction, setup, AddIndexComponentsTransaction,
};

use borsh::BorshDeserialize;
use open_index::state::{Component, Controller, Index, IndexMints, Protocol};
use solana_sdk::{
    clock::sysvar,
    instruction::InstructionError,
    msg,
    program_pack::{IsInitialized, Pack},
    rent::Rent,
    signature::Keypair,
    syscalls,
    system_instruction::create_account,
    transaction::TransactionError,
};

use {solana_program_test::tokio, solana_sdk::signature::Signer};
#[tokio::test]
async fn test_add_index_components() {
    let _setup = setup().await;
    let program_id = _setup.program_id;
    let manager = Keypair::new();
    let mint_1 = Keypair::new();
    let mint_2 = Keypair::new();
    // Initialize protocol
    let init_protocol_instruction = init_protocol_transaction(&_setup).await;
    let _ = _setup
        .banks_client
        .process_transaction(init_protocol_instruction.transaction.clone())
        .await;
    // Initialize Controller
    let controller_id = 1;
    let init_controller_tx = init_controller_transaction(controller_id, &_setup).await;
    let controller_pda = init_controller_tx.controller_pda;
    let _ = _setup
        .banks_client
        .process_transaction(init_controller_tx.transaction.clone())
        .await;
    // Create controller global  config tx
    let controller_global_tx = init_controller_global_config(10, &_setup).await;
    let _ = _setup
        .banks_client
        .process_transaction(controller_global_tx.transaction.clone())
        .await;
    // Create Index tx
    let mint = get_index_mint_pda(&program_id, &controller_pda, controller_id).0;
    let create_index_tx =
        create_index_transaction(1, controller_id, mint.clone(), manager.pubkey(), &_setup).await;
    let _ = _setup
        .banks_client
        .process_transaction(create_index_tx.transaction)
        .await;
    // Create mints
    let index_id = 1;
    let create_mint_1_transaction = create_mint_acccount_transaction(&mint_1, &_setup).await;
    let create_mint_2_transaction = create_mint_acccount_transaction(&mint_2, &_setup).await;
    let _ = _setup
        .banks_client
        .process_transaction(create_mint_1_transaction.transaction)
        .await;
    let _ = _setup
        .banks_client
        .process_transaction(create_mint_2_transaction.transaction)
        .await;
    let AddIndexComponentsTransaction {
        index_mints_data_pda,
        components,
        transaction,
    } = add_index_components_transaction(
        index_id,
        controller_id,
        vec![mint_1.pubkey(), mint_2.pubkey()],
        vec![10, 20],
        &_setup,
    )
    .await;
    let result = _setup.banks_client.process_transaction(transaction).await;
    assert!(result.is_err() == false);

    let index_mints_data_account = _setup
        .banks_client
        .get_account(index_mints_data_pda)
        .await
        .unwrap()
        .unwrap();

    let component_1 = components.get(0).unwrap();
    let component_2 = components.get(1).unwrap();
    let component_1_data_account = _setup
        .banks_client
        .get_account(component_1.clone())
        .await
        .unwrap()
        .unwrap();

    let component_2_data_account = _setup
        .banks_client
        .get_account(component_2.clone())
        .await
        .unwrap()
        .unwrap();
    let component_1_data = Component::try_from_slice(&component_1_data_account.data).unwrap();
    let component_2_data = Component::try_from_slice(&component_2_data_account.data).unwrap();

    let index_mints_data = IndexMints::try_from_slice(&index_mints_data_account.data).unwrap();
    let index_mint_1 = index_mints_data.mints.get(0).unwrap();
    let index_mint_2 = index_mints_data.mints.get(1).unwrap();
    assert!(index_mints_data.is_initialized());
    assert_eq!(index_mints_data.mints.len(), 2);
    assert_eq!(index_mint_1.clone(), mint_1.pubkey());
    assert_eq!(index_mint_2.clone(), mint_2.pubkey());

    assert!(component_1_data.is_initialized());
    assert_eq!(component_1_data.mint, *index_mint_1);
    assert_eq!(component_1_data.uints, 10);
    assert!(component_2_data.is_initialized());
    assert_eq!(component_2_data.mint, *index_mint_2);
    assert_eq!(component_2_data.uints, 20);
}
