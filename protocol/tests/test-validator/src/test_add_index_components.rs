
use anyhow::Result;
use borsh::BorshSerialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    address_lookup_table::instruction::create_lookup_table, commitment_config::CommitmentConfig, instruction::{AccountMeta, Instruction}, message::Message, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signer}, system_program, transaction::Transaction
};
use std::env;
use std::{path::PathBuf, str::FromStr};

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

use crate::{get_open_index_program_id, setup};

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn test_add_index_components() -> Result<()> {
    let _context = setup();
    let payer = _context.payer;
    let program_id = _context.open_index_program_id; 
    let client = _context.client.clone();
    let recent_blockhashes = tokio::task::spawn_blocking(move || client.get_latest_blockhash()).await??;
 
    // Connect to local validator 
    // let recent_slot = client.get_slot_with_commitment(CommitmentConfig::finalized())?;
    // let manager = Keypair::new();
    // let mint_1 = Keypair::new();
    // let mint_2 = Keypair::new();
    // let mint_3 = Keypair::new();
    // let mint_4 = Keypair::new();
    // let mint_5 = Keypair::new();
    // let mint_6 = Keypair::new();
    // let mint_7 = Keypair::new();
    // let mint_8 = Keypair::new();
    //
   let init_protocol_instruction = init_protocol_transaction(&payer, program_id, recent_blockhashes);
   _context.client.send_and_confirm_transaction(&init_protocol_instruction)?;

 
        // Load keypairs
    // let payer_path =env::var("HOME").map_err(|e| anyhow::anyhow!("Failed to get HOME env var: {}", e))?;
    // let payer = read_keypair_file(format!("{}/.config/solana/id.json", payer_path)).map_err(|e| anyhow::anyhow!("Failed to read payer keypair: {}", e))?;
    // let recent_slot = client.get_slot_with_commitment(CommitmentConfig::finalized())?;

    // //

    // let init_protocol_instruction =
    // init_protocol_transaction(&_setup.payer, _setup.program_id, _setup.recent_blockhashes);
    // //

    // let (create_account_instruction, lookup_table_address) =
    // create_lookup_table(payer.pubkey(), payer.pubkey(), recent_slot);
   
    // let latest_blockhash = client.get_latest_blockhash()?;
    // let transaction = Transaction::new_signed_with_payer(
    //     &[create_account_instruction],
    //     Some(&payer.pubkey()),
    //     &[&payer],
    //     latest_blockhash
    // );

    // let sig = client.send_and_confirm_transaction(&transaction)?;

    // println!("{:?} sig ",sig);

    Ok(())
}
