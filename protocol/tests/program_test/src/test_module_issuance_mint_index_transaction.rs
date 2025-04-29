use openindex_sdk::openindex::transaction::{
    create_mint_acccount_transaction, create_token_account_transaction, mint_to_transaction,
};
use solana_program_test::tokio;
use solana_sdk::{program_pack::Pack, signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::{instruction::mint_to_checked, state::Account};

use crate::{setup, Setup};

#[tokio::test]
async fn test_module_issuance_mint_index_transaction() {
    let _setup: Setup = setup().await;
    let payer = &_setup.payer;
    let program_id = _setup.program_id;
    let mint = Keypair::new();

    let create_mint_tx = create_mint_acccount_transaction(
        &_setup.payer,
        &mint,
        _setup.recent_blockhashes,
        &_setup.rent,
    );
    let result = _setup
        .banks_client
        .process_transaction(create_mint_tx)
        .await;
    assert!(result.is_err() == false);

    let create_token_account_tx = create_token_account_transaction(
        &_setup.payer,
        _setup.payer.pubkey(),
        _setup.payer.pubkey(),
        mint.pubkey(),
        _setup.recent_blockhashes,
    );
    let result = _setup
        .banks_client
        .process_transaction(create_token_account_tx)
        .await;
    assert!(result.is_err() == false);
    let token_account = get_associated_token_address_with_program_id(
        &payer.pubkey(),
        &mint.pubkey(),
        &spl_token::ID,
    );
    let mint_amount = 100_000_000;
    let mint_to_tx = mint_to_transaction(
        &payer,
        payer.pubkey(),
        payer.pubkey(),
        mint_amount,
        mint.pubkey(),
        token_account,
        _setup.recent_blockhashes,
    )
    .unwrap();

    let result = _setup.banks_client.process_transaction(mint_to_tx).await;
    assert!(result.is_err() == false);

    let account = _setup
        .banks_client
        .get_account(token_account.clone())
        .await
        .unwrap()
        .unwrap();
    let token_account = Account::unpack(&account.data).unwrap();

    assert_eq!(token_account.amount, mint_amount);

    //Todo!
    // complete module_issuance_mint_index_transaction
    // create token accounts for each of the component tokens and mint to user so as to deposit
}
