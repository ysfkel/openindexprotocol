use std::str::FromStr;

use crate::TestContext;
use openindex_sdk::openindex::{
    pda::{find_controller_address, find_index_mint_address},
    transaction::{create_token_account_transaction, mint_to_transaction, mint_transaction},
};
use solana_sdk::{program_pack::Pack, pubkey::Pubkey, signature::Keypair};
use solana_sdk::{signature::Signer, transaction::Transaction};
use spl_associated_token_account::{create_associated_token_account, get_associated_token_address};
use spl_token::state::Account;

pub struct ProcessMintResult {
    pub index_id: u64,
    pub controller_id: u64,
    pub token_account: Pubkey,
    pub token_accounts: Vec<Pubkey>,
}

pub async fn process_mint(
    index_mint_amount: u64,
    tokens_mint_amount: u64,
    controller_id: u64,
    index_id: u64,
    mints: Vec<Pubkey>,
    _setup: &TestContext,
) -> ProcessMintResult {
    let open_index_program_id = _setup.openindex_program_id;
    let payer = &_setup.payer;
    let recent_blockhashes = _setup.client.get_latest_blockhash().unwrap().clone();
    let mut token_accounts: Vec<Pubkey> = vec![];
    for mint in mints.iter() {
        let ata = get_associated_token_address(&payer.pubkey(), mint);

        // 1) Create the ATA
        let ix = create_associated_token_account(&payer.pubkey(), &payer.pubkey(), mint);

        let result =
            _setup
                .client
                .send_and_confirm_transaction(&Transaction::new_signed_with_payer(
                    &[ix],
                    Some(&payer.pubkey()),
                    &[payer],
                    recent_blockhashes,
                ));

        let token_account = get_associated_token_address(&payer.pubkey(), &mint);

        assert_eq!(ata, token_account);

        println!("user component token account {:?} -> ", token_account.clone());

        // mint to the token account
        let mint_to_tx = mint_to_transaction(
            &payer,
            tokens_mint_amount,
            mint.clone(),
            token_account.clone(),
            recent_blockhashes,
        )
        .unwrap();

        let result = _setup.client.send_and_confirm_transaction(&mint_to_tx);

        token_accounts.push(token_account);

        // execute the transaction
        let account = _setup.client.get_account(&token_account).unwrap();
        let token_account = Account::unpack(&account.data).unwrap();
    }

    let controller_pda = find_controller_address(&open_index_program_id, controller_id).0;
    let mint = find_index_mint_address(&open_index_program_id, &controller_pda, index_id).0;

    let create_token_account_tx = create_token_account_transaction(
        &_setup.payer,
        _setup.payer.pubkey(),
        _setup.payer.pubkey(),
        mint,
        recent_blockhashes,
    );

    let sig = _setup
        .client
        .send_and_confirm_transaction(&create_token_account_tx)
        .unwrap();

    let token_account: Pubkey = get_associated_token_address(&payer.pubkey(), &mint);

    let token_account_raw = _setup.client.get_account(&token_account).unwrap();
    let token_account_data =
        spl_token::state::Account::unpack(&token_account_raw.data.as_ref()).unwrap();

    let mint_account = _setup.client.get_account(&mint).unwrap();
    let mint_account_data = spl_token::state::Mint::unpack(&mint_account.data.as_ref()).unwrap();

    assert_eq!(token_account_data.mint, mint);

    println!("user index token_account {:?} ",token_account);

    assert!(mints.len() > 0);
    assert_eq!(mints.len(), token_accounts.len());

    let mint_index_tx = mint_transaction(
        index_mint_amount,
        payer,
        _setup.openindex_program_id,
        index_id,
        controller_id,
        token_account,
        recent_blockhashes,
        mints.clone(),
        token_accounts.clone(),
    );

    let result = _setup.client.send_and_confirm_transaction(&mint_index_tx);
    println!("{:?}", result);

    ProcessMintResult {
        index_id,
        controller_id,
        token_account,
        token_accounts,
    }
}
