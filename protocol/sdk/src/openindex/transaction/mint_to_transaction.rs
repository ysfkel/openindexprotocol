use crate::openindex::error::TransactionBuilderError;
use solana_sdk::{
    hash::Hash, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
};
use spl_token::instruction::mint_to_checked;

pub fn mint_to_transaction(
    payer: &Keypair,
    amount: u64,
    mint: Pubkey,
    token_account: Pubkey,
    recent_blockhashes: Hash,
) -> Result<Transaction, TransactionBuilderError> {
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
