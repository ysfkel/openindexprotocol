use openindex::entrypoint::process_instruction;
use solana_program::example_mocks::solana_sdk::{system_instruction, sysvar::recent_blockhashes};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    rent::Rent,
    sysvar::Sysvar,
    transaction::Transaction,
};
use spl_token::state::Mint;
use std::str::FromStr;

use {
    solana_program::pubkey::Pubkey,
    solana_program_test::{processor, tokio, ProgramTest},
    solana_sdk::signature::{Keypair, Signer},
    spl_token::state::Account as TokenAccount,
};

#[tokio::test]
async fn test_controller() {
    /// TransferTokens11111111111111111111111111111111
    let program_id = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    let source = Keypair::new();
    let mint = Keypair::new();
    let destination = Keypair::new();
    let (authority_pubkey, _) = Pubkey::find_program_address(&[b"authority"], &program_id);

    let mut program_test = ProgramTest::default();
    //  let mut program_test = ProgramTest::new("openindex", program_id,  processor!(process_instruction);
    // program_test.add_program("openindex", program_id,  processor!(process_instruction));

    let (mut banks_client, payer, recent_blockhashes) = program_test.start().await;

    let rent = Rent::default();
    let decimals = 9;

    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &mint.pubkey(),
                rent.minimum_balance(Mint::LEN),
                Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &mint.pubkey(),
                &payer.pubkey(),
                None,
                decimals,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        recent_blockhashes,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &source.pubkey(),
                rent.minimum_balance(TokenAccount::LEN),
                TokenAccount::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(
                &spl_token::id(),
                &source.pubkey(),
                &mint.pubkey(),
                &authority_pubkey,
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
        &[&payer, &source],
        recent_blockhashes,
    );
    banks_client.process_transaction(transaction).await.unwrap();

    let mint_amount = 1_000_000_000;
    /// mint to source
    let transaction = Transaction::new_signed_with_payer(
        &[spl_token::instruction::mint_to(
            &spl_token::id(),
            &mint.pubkey(),
            &source.pubkey(),
            &payer.pubkey(),
            &[],
            1_000_000_000,
        )
        .unwrap()],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhashes,
    );
    banks_client.process_transaction(transaction).await.unwrap();

    let account = banks_client
        .get_account(source.pubkey())
        .await
        .unwrap()
        .unwrap();
    let token_account = TokenAccount::unpack(&account.data).unwrap();

    //     assert_eq!(token_account.amount, mint_amount);
    // use this for calling my program
    // let transaction = Transaction::new_signed_with_payer(
    //     &[
    //         Instruction::new_with_bincode(program_id, &(),
    //        vec![
    //           AccountMeta::new(source.pubkey(), false),
    //           AccountMeta::new(destination.pubkey(), false)
    //        ]
    //     )
    //     ],
    //     Some(&payer.pubkey()),
    //     &[&payer, &source],
    //     recent_blockhashes,
    // );
    // banks_client.process_transaction(transaction).await.unwrap();
}
