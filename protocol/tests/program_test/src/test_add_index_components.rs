use crate::setup;

use borsh::BorshDeserialize;
use open_index::state::{Component, Controller, Index, IndexMints, Protocol};
use open_index_lib::{
    pda::{
        find_component_address, find_component_vault_address, find_controller_address,
        find_index_address, find_index_mint_address, find_index_mints_data_address,
    },
    transaction::{
        add_index_components_transaction, create_index_transaction,
        create_lookup_table_transaction, create_mint_acccount_transaction,
        init_controller_global_config_transaction, init_controller_transaction,
        init_protocol_transaction,
    },
};
use serde::Deserialize;
use solana_sdk::{
    account::Account,
    address_lookup_table::instruction::derive_lookup_table_address,
    clock::{sysvar, Clock},
    instruction::InstructionError,
    message::AddressLookupTableAccount,
    msg,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    rent::Rent,
    signature::Keypair,
    signer::SeedDerivable,
    syscalls,
    system_instruction::create_account,
    sysvar::{Sysvar, SysvarId},
    transaction::TransactionError,
};
use spl_associated_token_account::get_associated_token_address_with_program_id;

use {solana_program_test::tokio, solana_sdk::signature::Signer};
#[tokio::test]
async fn test_add_index_components() {
    let _setup = setup().await;
    let program_id = _setup.program_id;
    let manager = Keypair::new();
    let mint_1 = Keypair::new();
    let mint_2 = Keypair::new();
    let mint_3 = Keypair::new();
    let mint_4 = Keypair::new();
    let mint_5 = Keypair::new();
    let mint_6 = Keypair::new();
    let mint_7 = Keypair::new();
    // Initialize protocol
    let init_protocol_instruction =
        init_protocol_transaction(&_setup.payer, _setup.program_id, _setup.recent_blockhashes);
    let _ = _setup
        .banks_client
        .process_transaction(init_protocol_instruction.clone())
        .await;
    // Initialize Controller
    let controller_id = 1;
    let init_controller_tx = init_controller_transaction(
        &_setup.payer,
        _setup.program_id,
        controller_id,
        _setup.recent_blockhashes,
    );

    let _ = _setup
        .banks_client
        .process_transaction(init_controller_tx.clone())
        .await;
    // Create controller global  config tx
    let controller_global_tx = init_controller_global_config_transaction(
        &_setup.payer,
        _setup.program_id,
        10,
        _setup.recent_blockhashes,
    );

    let _ = _setup
        .banks_client
        .process_transaction(controller_global_tx.clone())
        .await;

    let controller_pda = find_controller_address(&program_id, controller_id).0;

    // Create Index tx
    let mint = find_index_mint_address(&program_id, &controller_pda, controller_id).0;

    let create_index_tx = create_index_transaction(
        &_setup.payer,
        _setup.program_id,
        1,
        controller_id,
        mint,
        manager.pubkey(),
        _setup.recent_blockhashes,
    );

    let _ = _setup
        .banks_client
        .process_transaction(create_index_tx)
        .await;
    // Create mints
    let index_id = 1;

    let create_mint_1_transaction = create_mint_acccount_transaction(
        &_setup.payer,
        &mint_1,
        _setup.recent_blockhashes,
        &_setup.rent,
    );
    let create_mint_2_transaction = create_mint_acccount_transaction(
        &_setup.payer,
        &mint_2,
        _setup.recent_blockhashes,
        &_setup.rent,
    );
    let create_mint_3_transaction = create_mint_acccount_transaction(
        &_setup.payer,
        &mint_3,
        _setup.recent_blockhashes,
        &_setup.rent,
    );
    let create_mint_4_transaction = create_mint_acccount_transaction(
        &_setup.payer,
        &mint_4,
        _setup.recent_blockhashes,
        &_setup.rent,
    );
    let create_mint_5_transaction = create_mint_acccount_transaction(
        &_setup.payer,
        &mint_5,
        _setup.recent_blockhashes,
        &_setup.rent,
    );
    let create_mint_6_transaction = create_mint_acccount_transaction(
        &_setup.payer,
        &mint_6,
        _setup.recent_blockhashes,
        &_setup.rent,
    );
    let create_mint_7_transaction = create_mint_acccount_transaction(
        &_setup.payer,
        &mint_7,
        _setup.recent_blockhashes,
        &_setup.rent,
    );

    let _ = _setup
        .banks_client
        .process_transaction(create_mint_1_transaction)
        .await;
    let _ = _setup
        .banks_client
        .process_transaction(create_mint_2_transaction)
        .await;
    let _ = _setup
        .banks_client
        .process_transaction(create_mint_3_transaction)
        .await;
    let _ = _setup
        .banks_client
        .process_transaction(create_mint_4_transaction)
        .await;
    let _ = _setup
        .banks_client
        .process_transaction(create_mint_5_transaction)
        .await;
    let _ = _setup
        .banks_client
        .process_transaction(create_mint_6_transaction)
        .await;
    let _ = _setup
        .banks_client
        .process_transaction(create_mint_7_transaction)
        .await;

    let mints = vec![
        mint_1.pubkey(),
        mint_2.pubkey(),
        mint_3.pubkey(),
        mint_4.pubkey(),
        mint_5.pubkey(),
        mint_6.pubkey(),
        mint_7.pubkey(),
    ];

    let transaction = add_index_components_transaction(
        &_setup.payer,
        _setup.program_id,
        index_id,
        controller_id,
        _setup.recent_blockhashes.clone(),
        vec![
            mint_1.pubkey(),
            mint_2.pubkey(),
            mint_3.pubkey(),
            mint_4.pubkey(),
            mint_5.pubkey(),
            mint_6.pubkey(),
            mint_7.pubkey(),
        ],
        vec![10, 20, 30, 40, 50, 60, 70],
    );

    let result = _setup.banks_client.process_transaction(transaction).await;
    assert!(result.is_err() == false);

    // Get component pda
    let controller_pda = find_controller_address(&program_id, controller_id).0;
    let index_pda = find_index_address(&program_id, &controller_pda, index_id).0;
    let mut components: Vec<Pubkey> = vec![];
    for mint in mints.iter() {
        let component_pda = find_component_address(&program_id, &index_pda, mint).0;
        components.push(component_pda);
    }
    let index_mints_data_pda =
        find_index_mints_data_address(&program_id, &controller_pda, index_id).0;

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
    assert_eq!(index_mints_data.mints.len(), mints.len());
    assert_eq!(index_mint_1.clone(), mint_1.pubkey());
    assert_eq!(index_mint_2.clone(), mint_2.pubkey());
    assert!(component_1_data.is_initialized());
    assert_eq!(component_1_data.mint, *index_mint_1);
    assert_eq!(component_1_data.uints, 10);
    assert!(component_2_data.is_initialized());
    assert_eq!(component_2_data.mint, *index_mint_2);
    assert_eq!(component_2_data.uints, 20);
}
