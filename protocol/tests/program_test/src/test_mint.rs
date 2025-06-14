use borsh::BorshDeserialize;
use openindex_sdk::{
    openindex::{
        state::Component,
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
    process_init_controller, process_init_module, process_init_protocol, process_mint, setup,
    ProcessAddIndexComponentsResult, ProcessCreateIndexResult, ProcessInitControllerResult,
    ProcessMintResult, Setup,
};
use spl_token::state::Account as TokenAccount;

#[tokio::test]
async fn test_mint() {
    let _setup: Setup = setup().await;
    let payer = &_setup.payer;
    let open_index_program_id = _setup.program_id;
    let issuance_program_id = _setup.issuance_program_id;
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

    let components_count = 2;
    let units: Vec<_> = (0..components_count).map(|i| (i as u64 + 10)).collect();

    let ProcessAddIndexComponentsResult {
        index_id,
        controller_id,
        mints,
        units,
        result,
    } = process_add_index_components(
        index_id,
        controller_id,
        manager.pubkey(),
        components_count,
        units,
        &_setup,
    )
    .await;

    // create user's token accounts for each mint and mint tokens to user
    let mint_amount = 1000;
    let ProcessMintResult {
        index_id,
        controller_id,
        token_accounts,
        token_account,
        result,
    } = process_mint(
        mint_amount,
        100_000_000,
        controller_id,
        index_id,
        mints.clone(),
        &_setup,
    )
    .await;
    assert!(result.is_err() == false);

    //verify that components token amounts were transfered to each component token ata vault
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
        println!(
            "index vault token accout.amoun {:?} == tx amount {:?}",
            token_account.amount, amount
        );
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
