use crate::{setup, Setup};
use solana_sdk::{
    signature::Keypair, signer::Signer, system_instruction::create_account,
    transaction::Transaction,
};

pub struct InitAccountTransaction {
    pub transaction: Transaction,
}
pub fn create_acccount_transaction(
    account: Keypair,
    lamports: u64,
    _setup: &Setup,
) -> InitAccountTransaction {
    let create_account_instruction = create_account(
        &_setup.payer.pubkey(),
        &account.pubkey(),
        lamports,
        lamports,
        &spl_token::id(),
    );

    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction],
        Some(&_setup.payer.pubkey()),
        &[&_setup.payer, &account],
        _setup.recent_blockhashes.clone(),
    );

    InitAccountTransaction { transaction }
}
