use solana_sdk::{
    hash::Hash, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
};
use spl_associated_token_account::instruction::create_associated_token_account;

pub fn create_token_account_transaction(
    payer: &Keypair,
    funding: Pubkey,
    wallet_address: Pubkey,
    mint: Pubkey,
    recent_blockhashes: Hash,
) -> Transaction {
    let instruction =
        create_associated_token_account(&funding, &wallet_address, &mint, &spl_token::ID);

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhashes,
    )
}
