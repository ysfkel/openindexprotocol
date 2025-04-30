use borsh::BorshDeserialize;
use openindex::state::Component;
use openindex_sdk::{
    issuance::transaction::mint_index_transaction,
    openindex::{
        pda::{
            find_component_address, find_component_vault_address, find_controller_address,
            find_index_address, find_index_mint_address,
        },
        transaction::{
            create_mint_acccount_transaction, create_token_account_transaction, mint_to_transaction,
        },
    },
};
use solana_program_test::tokio;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey, signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::{instruction::mint_to_checked, state::Account};

use crate::{
    process_add_index_components, process_controller_global_config, process_create_index,
    process_init_controller, process_init_module, process_init_protocol, setup,
    ProcessAddIndexComponentsResult, ProcessCreateIndexResult, ProcessInitControllerResult, Setup,
};
use spl_token::state::Account as TokenAccount;

#[tokio::test]
async fn test_module_issuance_mint_index_transaction() {
    let _setup: Setup = setup().await;
    let payer = &_setup.payer;
    let open_index_program_id = _setup.program_id;
    let issuance_program_id = _setup.issuance_program_id;
    // let mint = Keypair::new();
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

    let _ = process_init_module(_setup.issuance_program_id, &_setup).await;

    let ProcessAddIndexComponentsResult {
        index_id,
        controller_id,
        mints,
        amounts,
        result,
    } = process_add_index_components(index_id, controller_id, manager.pubkey(), 4, &_setup).await;

    // create user's token accounts for each mint and mint tokens to user
    let mut token_accounts: Vec<Pubkey> = vec![];
    for mint in mints.iter() {
        let create_token_account_tx = create_token_account_transaction(
            &_setup.payer,
            _setup.payer.pubkey(),
            _setup.payer.pubkey(),
            mint.clone(),
            _setup.recent_blockhashes,
        );
        // execute the transaction
        let result = _setup
            .banks_client
            .process_transaction(create_token_account_tx)
            .await;
        assert!(result.is_err() == false);
        // create mint transaction
        let token_account =
            get_associated_token_address_with_program_id(&payer.pubkey(), &mint, &spl_token::ID);
        let mint_amount = 100_000_000;

        // mint to the token account
        let mint_to_tx = mint_to_transaction(
            &payer,
            payer.pubkey(),
            payer.pubkey(),
            mint_amount,
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

        assert_eq!(token_account.amount, mint_amount);
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

    let mint_amount = 100;
    let mint_index_tx = mint_index_transaction(
        mint_amount,
        payer,
        _setup.issuance_program_id,
        _setup.program_id,
        index_id,
        controller_id,
        token_account,
        _setup.recent_blockhashes,
        mints.clone(),
        token_accounts.clone(),
    );

    let result = _setup.banks_client.process_transaction(mint_index_tx).await;
    assert!(result.is_err() == false);
    
    // verify that components token amounts were transfered to each component token ata vault 
    for (index, mint) in mints.iter().enumerate() {
        let index_account = find_index_address(&open_index_program_id, &controller_pda, index_id).0;
        let component_pda = find_component_address(&open_index_program_id, &index_account, mint).0;
        let vault_pda =
            find_component_vault_address(&open_index_program_id, &index_account, mint).0;
        let vault_ata =
            get_associated_token_address_with_program_id(&vault_pda, mint, &spl_token::ID);

        let component_account = _setup
            .banks_client
            .get_account(component_pda)
            .await
            .unwrap()
            .unwrap();
        
        let account = _setup
            .banks_client
            .get_account(vault_ata)
            .await
            .unwrap()
            .unwrap();
        
        let component = Component::try_from_slice(&component_account.data).unwrap();
        let token_account = TokenAccount::unpack(&account.data).unwrap();
        let amount = component.uints * mint_amount;
        println!("index vault token accout.amoun {:?} == tx amount {:?}",token_account.amount, amount);
        assert_eq!(token_account.amount, amount);
    }

    // verify user index token balance
    let account = _setup
            .banks_client
            .get_account(token_account)
            .await
            .unwrap()
            .unwrap();

    let token_account = TokenAccount::unpack(&account.data).unwrap();
     assert_eq!(token_account.amount, mint_amount);

}
