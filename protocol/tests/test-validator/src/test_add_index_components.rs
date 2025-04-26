use crate::{get_open_index_program_id, setup};
use open_index::state::{Controller, ControllerGlobalConfig};
use open_index_lib::pda::find_controller_global_config_address;
use solana_client::rpc_request::RpcError;
use solana_sdk::account::Account;
use std::panic;
use {
    anyhow::Result,
    borsh::{BorshDeserialize, BorshSerialize},
    open_index::state::Protocol,
    open_index_lib::{
        pda::{
            find_component_address, find_component_vault_address, find_controller_address,
            find_index_address, find_index_mint_address, find_index_mints_data_address,
            find_protocol_address,
        },
        transaction::{
            add_index_components_transaction, create_index_transaction,
            create_lookup_table_transaction, create_mint_acccount_transaction,
            init_controller_global_config_transaction, init_controller_transaction,
            init_protocol_transaction,
        },
    },
    solana_client::client_error::ClientErrorKind,
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        address_lookup_table::instruction::create_lookup_table,
        commitment_config::CommitmentConfig,
        instruction::{AccountMeta, Instruction},
        message::Message,
        pubkey::Pubkey,
        signature::{Keypair, Signer, read_keypair_file},
        system_program,
        transaction::Transaction,
    },
    std::env,
    std::{path::PathBuf, str::FromStr},
};

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn test_add_index_components() -> Result<()> {
    let _context = setup();
    let payer = _context.payer;
    let program_id = _context.open_index_program_id;
    let client = _context.client.clone();
    let recent_blockhashes =
        tokio::task::spawn_blocking(move || client.get_latest_blockhash()).await??;
    let protocol_address = find_protocol_address(&program_id).0;
    let controller_global_config_address = find_controller_global_config_address(&program_id).0;

    // Initialize protocol if not already initialized
    let mut protocol_account_result = _context.client.get_account(&protocol_address);
    let mut protocol_account = Account::default();

    if let Err(e) = protocol_account_result {
        if let ClientErrorKind::RpcError(rpc_error) = e.kind() {
            if let RpcError::ForUser(msg) = rpc_error {
                let init_protocol_tx =
                    init_protocol_transaction(&payer, program_id, recent_blockhashes);
                _context
                    .client
                    .send_and_confirm_transaction(&init_protocol_tx)
                    .unwrap();
                protocol_account = _context.client.get_account(&protocol_address).unwrap();
            }
        } else {
            panic!("{:?}", e);
        }
    } else {
        protocol_account = protocol_account_result.unwrap();
    }

    assert!(protocol_account.lamports > 0);
    let protocol_data = Protocol::try_from_slice(&protocol_account.data)?;
    assert!(protocol_data.initialized);

    // Initialize Controller global config if not already initialized
    let mut controller_global_config_result = _context
        .client
        .get_account(&controller_global_config_address);
    let mut controller_global_config_account = Account::default();

    if let Err(e) = controller_global_config_result {
        if let ClientErrorKind::RpcError(rpc_error) = e.kind() {
            if let RpcError::ForUser(msg) = rpc_error {
                let controller_global_config_tx = init_controller_global_config_transaction(
                    &payer,
                    program_id,
                    10,
                    recent_blockhashes,
                );

                _context
                    .client
                    .send_and_confirm_transaction(&controller_global_config_tx)
                    .unwrap();

                controller_global_config_account = _context
                    .client
                    .get_account(&controller_global_config_address)
                    .unwrap();
            }
        } else {
            panic!("{:?}", e);
        }
    } else {
        controller_global_config_account = controller_global_config_result.unwrap();
    }
    assert!(controller_global_config_account.lamports > 0);
    let controller_global_config_data =
        ControllerGlobalConfig::try_from_slice(&controller_global_config_account.data)?;
    assert!(controller_global_config_data.initialized);

    // Create controller
    let controller_address =
        find_controller_address(&program_id, protocol_data.get_next_controller_id()).0;
    let init_controller_tx = init_controller_transaction(
        &payer,
        program_id,
        protocol_data.get_next_controller_id(),
        recent_blockhashes,
    );
    _context
        .client
        .send_and_confirm_transaction(&init_controller_tx)
        .unwrap();

    let controller_account = _context.client.get_account(&controller_address).unwrap();
    let controller_data = Controller::try_from_slice(&controller_account.data)?;
    assert!(controller_data.initialized);
    // Create controller
    // let controller_id = protocol_data.get_next_controller_id();
    // let init_controller_tx =
    //     init_controller_transaction(&payer, program_id, controller_id, recent_blockhashes);
    // _context
    //     .client
    //     .send_and_confirm_transaction(&init_controller_tx)?;
    // // Create controller global  config tx
    // let controller_global_tx =
    //     init_controller_global_config_transaction(&payer, program_id, 10, recent_blockhashes);

    Ok(())
}
