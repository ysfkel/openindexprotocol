use crate::error::TransactionBuilderError;
use solana_sdk::{
    hash::Hash, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token::instruction::mint_to_checked;

pub fn mint_to_transaction(
    payer: &Keypair,
    funding: Pubkey,
    wallet_address: Pubkey,
    amount: u64,
    mint: Pubkey,
    token_account: Pubkey,
    recent_blockhashes: Hash,
) -> Result<Transaction, TransactionBuilderError> {
    let instruction =
        create_associated_token_account(&funding, &wallet_address, &mint, &spl_token::ID);

    let instruction: solana_sdk::instruction::Instruction = mint_to_checked(
        &spl_token::ID,
        &mint,
        &token_account,
        &payer.pubkey(),
        &[&payer.pubkey()],
        amount,
        9,
    )?;

    Ok(Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhashes,
    ))
}
