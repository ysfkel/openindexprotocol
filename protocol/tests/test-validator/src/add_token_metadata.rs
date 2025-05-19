use std::str::FromStr;

use anyhow::Result;
use mpl_token_metadata::instructions::{CreateV1, CreateV1InstructionArgs};
use mpl_token_metadata::types::{TokenStandard, PrintSupply};
use openindex_sdk::openindex::pda::{find_controller_address, find_index_mint_address};
use solana_program::{pubkey::Pubkey, system_program, sysvar};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    commitment_config::CommitmentConfig,
};
use crate::setup;

fn find_metadata_pda(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint.as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
}


pub fn add_metadata(mint_pubkey: Pubkey, meta: (String, String)) -> Result<()> {

    let _setup = setup();
    let payer = _setup.payer;
 
   
let (metadata_pda, _) = find_metadata_pda(&mint_pubkey);

 
    let args = CreateV1InstructionArgs {
        name: meta.0.to_string(),
        symbol: meta.1.to_string(),
        uri: "https://example.com/metadata.json".to_string(),
        seller_fee_basis_points: 500, // 5%
        primary_sale_happened: false,
        is_mutable: true,
        token_standard: TokenStandard::Fungible,
        collection: None,
        uses: None,
        collection_details: None,
        creators: None,
        rule_set: None,
        decimals: Some(9),
        print_supply: Some(PrintSupply::Zero),
    };
    
    let create_ix = CreateV1 {
        metadata: metadata_pda,
        master_edition: None,
        mint: (mint_pubkey, false),
        authority: payer.pubkey(),
        payer: payer.pubkey(),
        update_authority: (payer.pubkey(), true),
        system_program: system_program::ID,
        sysvar_instructions: sysvar::instructions::ID,
        spl_token_program: Some(spl_token::ID),
    };
    


let recent_blockhash = _setup.client.get_latest_blockhash()?;

let instruction = create_ix.instruction(args);
let transaction = Transaction::new_signed_with_payer(
    &[instruction],
    Some(&payer.pubkey()),
    &[&payer],
    recent_blockhash,
);

let restult = _setup.client.send_and_confirm_transaction(&transaction);
dbg!(restult);
// assert!(restult.is_err() == false);
// println!("Transaction Signature: {} {}", restult.unwrap(), meta.0);

Ok(())
}


// #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
pub async fn run() {

    println!("creating metadata account");

    let metadata = [(
        "Raydium".to_string(),
        "RAY".to_string()
    ),
    (
        "Orca".to_string(),
        "ORCA".to_string()
    )
    ];
 
    let mints = vec![
        Pubkey::from_str("FnbUFuu1PohJc3XhWjCZ5mE5PLouUUsGHJ2nFov1xmu3").unwrap(),
        Pubkey::from_str("AMNLnKBxUYtF76rUNJsBdVLcq8YkPRKErm3dL4okmnNm").unwrap(), 
    ];

    for (i, p) in mints.iter().enumerate() {
        let meta = metadata.get(i).unwrap().clone();
        add_metadata(p.clone(),meta);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
pub async fn run2() {
       println!("creating index metadata account");
      let _setup = setup();
    let program_id = &_setup.openindex_program_id.clone();
     let controller_address = find_controller_address(program_id, 1).0;
     let index_mint_address = find_index_mint_address(program_id, &controller_address, 1).0;
        add_metadata(index_mint_address,("DEFI PULSE".to_string(),"DPI".to_string() ));
   // find_index_mint_address(program_id, controller_account, index_id)
   // find_controller_address(program_id, controller_id)
}