use crate::{
    process_add_index_components, process_controller_global_config, process_create_index,
    process_init_controller, process_init_protocol, setup, BanksClientResult,
    ProcessAddIndexComponentsResult, ProcessCreateIndexResult, ProcessInitControllerResult, Setup,
};

use borsh::BorshDeserialize;
use openindex::state::{Component, Controller, Index, IndexMints, Protocol};
use openindex_sdk::openindex::{
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
use solana_program_test::BanksClientError;
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

    let _ = process_init_protocol(&_setup).await;

    let _ = process_controller_global_config(10, &_setup).await;

    let ProcessInitControllerResult {
        controller_id,
        controller_pda,
        result,
    } = process_init_controller(&_setup).await;

    let ProcessCreateIndexResult {
        index_id,
        controller_pda,
        result,
    } = process_create_index(controller_id, manager.pubkey(), &_setup).await;

    let ProcessAddIndexComponentsResult {
        index_id,
        controller_id,
        mints,
        amounts,
        result,
    } = process_add_index_components(index_id, controller_id, manager.pubkey(), 4, &_setup).await;

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
    let mint_1 = mints.get(0).unwrap().clone();
    let mint_2 = mints.get(1).unwrap().clone();
    assert_eq!(index_mint_1.clone(), mint_1);
    assert_eq!(index_mint_2.clone(), mint_2);
    assert!(component_1_data.is_initialized());
    assert_eq!(component_1_data.mint, *index_mint_1);
    let mint_1_amount = amounts.get(0).unwrap().clone();
    assert_eq!(component_1_data.uints, mint_1_amount);
    assert!(component_2_data.is_initialized());
    assert_eq!(component_2_data.mint, *index_mint_2);
    let mint_2_amount = amounts.get(1).unwrap().clone();
    assert_eq!(component_2_data.uints, mint_2_amount);
}
