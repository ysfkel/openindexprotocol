use solana_sdk::{
    hash::Hash, signature::Keypair, signer::Signer, system_instruction::create_account,
    transaction::Transaction,
};

pub fn create_acccount_transaction(
    payer: &Keypair,
    account: &Keypair,
    lamports: u64,
    recent_blockhashes: Hash,
) -> Transaction {
    let create_account_instruction = create_account(
        &payer.pubkey(),
        &account.pubkey(),
        lamports,
        lamports,
        &spl_token::id(),
    );

    Transaction::new_signed_with_payer(
        &[create_account_instruction],
        Some(&payer.pubkey()),
        &[payer, account],
        recent_blockhashes,
    )
}
