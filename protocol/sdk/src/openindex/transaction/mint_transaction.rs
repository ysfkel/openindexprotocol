use crate::openindex::{
    instruction::mint_instruction_with_dynamic_accounts,
    pda::{
        find_controller_address, find_index_address, find_index_mint_address,
        find_index_mint_authority_address, find_index_mints_data_address,
    },
};
use solana_sdk::{hash::Hash, transaction::Transaction};

use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub fn mint_transaction(
    amount: u64,
    payer: &Keypair,
    program_id: Pubkey,
    index_id: u64,
    controller_id: u64,
    token_account: Pubkey,
    recent_blockhashes: Hash,
    mints: Vec<Pubkey>,
    token_accounts: Vec<Pubkey>,
) -> Transaction {
    let controller_account = find_controller_address(&program_id, controller_id).0;
    let index_account = find_index_address(&program_id, &controller_account, index_id).0;
    let mint_account = find_index_mint_address(&program_id, &controller_account, index_id).0;

    let mint_authority_account =
        find_index_mint_authority_address(&program_id, &controller_account, index_id).0;

    let index_mints_data_account =
        find_index_mints_data_address(&program_id, &controller_account, index_id).0;
    let token_program_account = spl_token::ID;
    let instruction = mint_instruction_with_dynamic_accounts(
        payer.pubkey(),
        program_id,
        controller_account,
        mint_account,
        mint_authority_account,
        index_account,
        index_mints_data_account,
        token_account,
        token_program_account,
        mints,
        token_accounts,
        index_id,
        amount,
    );

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhashes,
    )
}
