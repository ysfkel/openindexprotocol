use std::str::FromStr;

use crate::{ProcessMintResult, Setup};
use openindex_sdk::openindex::{
    pda::{find_controller_address, find_index_mint_address},
    transaction::{create_token_account_transaction, mint_to_transaction, mint_transaction},
};
use solana_sdk::signature::Signer;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey, signature::Keypair};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::state::Account;

pub async fn process_mint(
    index_mint_amount: u64,
    tokens_mint_amount: u64,
    controller_id: u64,
    index_id: u64,
    mints: Vec<Pubkey>,
    _setup: &Setup,
) -> ProcessMintResult {
    let open_index_program_id = _setup.program_id;
    let payer = &_setup.payer;

    let mut token_accounts: Vec<Pubkey> = vec![];
    for mint in mints.iter() {
        let create_token_account_tx = create_token_account_transaction(
            &_setup.payer,
            _setup.payer.pubkey(),
            _setup.payer.pubkey(),
            mint.clone(),
            _setup.recent_blockhashes,
        );
        // execute create token account transaction
        let result = _setup
            .banks_client
            .process_transaction(create_token_account_tx)
            .await;
        assert!(result.is_err() == false);

        let token_account =
            get_associated_token_address_with_program_id(&payer.pubkey(), &mint, &spl_token::ID);

        // mint to the token account
        let mint_to_tx = mint_to_transaction(
            &payer,
            tokens_mint_amount,
            mint.clone(),
            token_account.clone(),
            _setup.recent_blockhashes,
        )
        .unwrap();

        let result = _setup.banks_client.process_transaction(mint_to_tx).await;
        assert!(result.is_err() == false);

        token_accounts.push(token_account);

        // execute the transaction
        let account = _setup
            .banks_client
            .get_account(token_account.clone())
            .await
            .unwrap()
            .unwrap();
        let token_account = Account::unpack(&account.data).unwrap();
    }

    let controller_pda = find_controller_address(&open_index_program_id, controller_id).0;
    let mint = find_index_mint_address(&open_index_program_id, &controller_pda, index_id).0;

    let create_token_account_tx = create_token_account_transaction(
        &_setup.payer,
        _setup.payer.pubkey(),
        _setup.payer.pubkey(),
        mint,
        _setup.recent_blockhashes,
    );

    let result = _setup
        .banks_client
        .process_transaction(create_token_account_tx)
        .await;

    let token_account =
        get_associated_token_address_with_program_id(&payer.pubkey(), &mint, &spl_token::ID);

    assert!(mints.len() > 0);
    assert_eq!(mints.len(), token_accounts.len());

    let mint_index_tx = mint_transaction(
        index_mint_amount,
        payer,
        _setup.program_id,
        index_id,
        controller_id,
        token_account,
        _setup.recent_blockhashes,
        mints.clone(),
        token_accounts.clone(),
    );

    let result = _setup.banks_client.process_transaction(mint_index_tx).await;

    ProcessMintResult {
        index_id,
        controller_id,
        token_account,
        token_accounts,
        result,
    }
}
