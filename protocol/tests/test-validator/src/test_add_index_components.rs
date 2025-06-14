use crate::{get_openindex_program_id, process_mint, setup, ProcessMintResult};
use openindex_sdk::openindex::{
    state::{Component, Controller, ControllerGlobalConfig, Index, IndexMints},
    pda::find_controller_global_config_address,
    transaction::add_index_components_versioned_transaction,
};
use solana_client::rpc_request::RpcError;
use solana_sdk::{
    account::Account,
    address_lookup_table::{instruction::derive_lookup_table_address, state::AddressLookupTable},
    message::AddressLookupTableAccount,
    program_pack::{IsInitialized, Pack},
};
use spl_associated_token_account::{
    get_associated_token_address, get_associated_token_address_with_program_id,
};
use spl_token::state::Account as TokenAccount;
use std::panic;
use {
    anyhow::Result,
    borsh::{BorshDeserialize, BorshSerialize},
    openindex::state::Protocol,
    openindex_sdk::openindex::{
        pda::{
            find_component_address, find_component_vault_address, find_controller_address,
            find_index_address, find_index_mint_address, find_index_mints_data_address,
            find_protocol_address,
        },
        transaction::{
            add_index_components_transaction, create_index_transaction,
            create_mint_acccount_transaction,
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
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
        transaction::Transaction,
    },
    std::env,
    std::{path::PathBuf, str::FromStr},
};

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_add_index_components() -> Result<()> {
    let _context = setup();
    let payer = &_context.payer;
    let program_id = _context.openindex_program_id;
    let client = &_context.client;

    let recent_blockhashes = client.get_latest_blockhash().unwrap();
    let protocol_address = find_protocol_address(&program_id).0;
    let controller_global_config_address = find_controller_global_config_address(&program_id).0;

    // Initialize protocol if not already initialized
    let protocol_data: Protocol = initialize_account_if_needed(client, &protocol_address, || {
        init_protocol_transaction(&payer, program_id, recent_blockhashes)
    })?;

    assert!(protocol_data.initialized);
    assert_eq!(protocol_data.owner, payer.pubkey());

    // Initialize Controller global config if not already initialized
    let controller_global_config_data: ControllerGlobalConfig =
        initialize_account_if_needed(client, &controller_global_config_address, || {
            init_controller_global_config_transaction(&payer, program_id, 10, recent_blockhashes)
        })?;

    // Create controller
    let controller_id = 1; //protocol_data.get_next_controller_id();
    let controller_address = find_controller_address(&program_id, controller_id).0;
    if (protocol_data.get_next_controller_id() == controller_id) {
        let init_controller_tx =
            init_controller_transaction(&payer, program_id, controller_id, recent_blockhashes);
        _context
            .client
            .send_and_confirm_transaction(&init_controller_tx)
            .unwrap();
    }

    let controller_account = client.get_account(&controller_address).unwrap();
    let controller_data = Controller::try_from_slice(&controller_account.data)?;
    assert!(controller_data.initialized);
    assert!(controller_data.owner == payer.pubkey());

    // Create Index tx
    let index_id = controller_data.next_index_id;
    println!("index_id {:?}", index_id);
    let index_address = find_index_address(&program_id, &controller_address, index_id).0;
    let mint = find_index_mint_address(&program_id, &controller_address, index_id).0;
    let manager = Keypair::new().pubkey();
    let create_index_tx = create_index_transaction(
        &payer,
        program_id,
        index_id,
        controller_id,
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
    let mut mints = Vec::<Pubkey>::new();
    let mut mints = vec![];
    let units = vec![1, 2];
    // Create mints
    for i in 1..=units.len() {
        let mint = Keypair::new();
        let create_mint_tx =
            create_mint_acccount_transaction(&payer, &mint, recent_blockhashes, &_context.rent);

        client.send_and_confirm_transaction(&create_mint_tx);

        mints.push(mint.pubkey());
        println!("mint {:?}", mint.pubkey());
    }

    let transaction = add_index_components_transaction(
        &payer,
        program_id,
        index_id,
        controller_id,
        recent_blockhashes,
        mints.clone(),
        units.clone(),
    );

    client.send_and_confirm_transaction(&transaction);
    let controller_address = find_controller_address(&program_id, controller_id).0;
    let index_pda = find_index_address(&program_id, &controller_address, index_id).0;
    let controller_account = client.get_account(&controller_address).unwrap();
    let mut components: Vec<Pubkey> = vec![];
    
    for mint in mints.iter() {
        let component_pda = find_component_address(&program_id, &index_pda, mint).0;
        components.push(component_pda);
    }
    let index_mints_data_pda =
        find_index_mints_data_address(&program_id, &controller_address, index_id).0;

    let mint_amount = 1_000_000_000; // i.e 1 considering 9 decimals

    let ProcessMintResult {
        index_id,
        controller_id,
        token_account,
        token_accounts,
    } = process_mint(
        mint_amount,
        1_0_000_000_000_000_000_000,
        controller_id,
        index_id,
        mints.clone(),
        &_context,
    )
    .await;

    let index_mints_data_account = client.get_account(&index_mints_data_pda).unwrap();
    let component_1 = components.get(0).unwrap();
    let component_2 = components.get(1).unwrap();
    let component_1_data_account = client.get_account(&component_1.clone()).unwrap();
    let component_2_data_account = client.get_account(&component_2.clone()).unwrap();
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
    let mint_1_amount = units.get(0).unwrap().clone();
    assert_eq!(component_1_data.uints, mint_1_amount);
    assert!(component_2_data.is_initialized());
    assert_eq!(component_2_data.mint, *index_mint_2);
    let mint_2_amount = units.get(1).unwrap().clone();
    assert_eq!(component_2_data.uints, mint_2_amount);

    // test mint
    //verify that components token amounts were transfered to each component token ata vault
    for (index, mint) in mints.iter().enumerate() {
        let index_account = find_index_address(
            &_context.openindex_program_id,
            &controller_address,
            index_id,
        )
        .0;
        let component_pda =
            find_component_address(&_context.openindex_program_id, &index_account, mint).0;
        let vault_pda =
            find_component_vault_address(&_context.openindex_program_id, &index_account, mint).0;
        let vault_ata = get_associated_token_address(&vault_pda, mint);

        let component_account = _context.client.get_account(&component_pda).unwrap();

        let account = _context.client.get_account(&vault_ata).unwrap();

        let component = Component::try_from_slice(&component_account.data).unwrap();
        let token_account = TokenAccount::unpack(&account.data).unwrap();
        let amount = component.uints * mint_amount;
        assert_eq!(token_account.amount, amount);
    }

    // verify user index token balance
    let account = _context.client.get_account(&token_account).unwrap();
    let token_account = TokenAccount::unpack(&account.data).unwrap();
    assert_eq!(token_account.amount, mint_amount);

    Ok(())
}

fn initialize_account_if_needed<T, F>(
    client: &RpcClient,
    address: &Pubkey,
    init_tx_fn: F,
) -> Result<T>
where
    T: BorshDeserialize,
    F: Fn() -> Transaction,
{
    match client.get_account(address) {
        Ok(account) => {
            let data = T::try_from_slice(&account.data)?;
            Ok(data)
        }
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
