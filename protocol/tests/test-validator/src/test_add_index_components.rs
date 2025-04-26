use crate::{get_open_index_program_id, setup};
use open_index::state::{Controller, ControllerGlobalConfig, Index};
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

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_add_index_components() -> Result<()> {
    let _context = setup();
    let payer = _context.payer;
    let program_id = _context.open_index_program_id;
    let client = &_context.client; 

    let recent_blockhashes =client.get_latest_blockhash().unwrap();
    let protocol_address = find_protocol_address(&program_id).0;
    let controller_global_config_address = find_controller_global_config_address(&program_id).0;

    // Initialize protocol if not already initialized
    let protocol_data: Protocol = initialize_account_if_needed(client, &protocol_address,|| {
        init_protocol_transaction(&payer, program_id, recent_blockhashes)
    })?;
    assert!(protocol_data.initialized);
    assert_eq!(protocol_data.owner, payer.pubkey());

    // Initialize Controller global config if not already initialized
    let controller_global_config_data: ControllerGlobalConfig = initialize_account_if_needed(client, &controller_global_config_address,|| {
        init_controller_global_config_transaction(&payer, program_id, 10,recent_blockhashes)
    })?;
    assert!(controller_global_config_data.initialized);
    assert_eq!(controller_global_config_data.max_index_components, 10);

    // Create controller
    let controller_address =
        find_controller_address(&program_id, protocol_data.get_next_controller_id()).0;
    let controller_id = protocol_data.get_next_controller_id();
    let init_controller_tx =
        init_controller_transaction(&payer, program_id, controller_id, recent_blockhashes);
    _context
        .client
        .send_and_confirm_transaction(&init_controller_tx)
        .unwrap();

    let controller_account = client.get_account(&controller_address).unwrap();
    let controller_data = Controller::try_from_slice(&controller_account.data)?;
    assert!(controller_data.initialized);
    assert!(controller_data.next_index_id == 1);
    assert!(controller_data.owner == payer.pubkey());

    // Create Index tx
    let index_id = controller_data.next_index_id;
    let index_address = find_index_address(&program_id, &controller_address, index_id).0;
    let mint = find_index_mint_address(&program_id, &controller_address, index_id).0;
    let manager =        Keypair::new().pubkey();
    let create_index_tx = create_index_transaction(
        &payer,
        program_id,
        index_id,
        controller_id,
        mint,
        manager,
        recent_blockhashes,
    );
    _context
        .client
        .send_and_confirm_transaction(&create_index_tx)
        .unwrap();
    let index_account = client.get_account(&index_address).unwrap();
    let index_data = Index::try_from_slice(&index_account.data)?;
    assert!(index_data.initialized);
    assert_eq!(index_data.manager, manager);
    assert!(index_data.owner == payer.pubkey());
   
    // todo
    // - create lookup table 
    // create versioned add components tx
    // submit and test
    Ok(())
}


fn initialize_account_if_needed<T,F>(
    client: &RpcClient,
    address: &Pubkey,
    init_tx_fn: F,
) -> Result<T>
where
    T: BorshDeserialize,
    F: FnOnce() -> Transaction,
{
    match client.get_account(address) {
        Ok(account) => {
            let data = T::try_from_slice(&account.data)?;
            Ok(data)
        },
        Err(e) => {
            if let ClientErrorKind::RpcError(RpcError::ForUser(msg)) = e.kind() {
                if msg.contains("AccountNotFound") {
                    let tx = init_tx_fn();
                    client.send_and_confirm_transaction(&tx)?;
                    let account = client.get_account(address)?;
                    let data = T::try_from_slice(&account.data)?;
                    Ok(data)
                } else {
                    Err(anyhow::anyhow!("Unexpected RPC error: {}", msg))
                }
            } else {
                Err(anyhow::anyhow!("Unexpected client error: {:?}", e))
            }
        }
    }
}
