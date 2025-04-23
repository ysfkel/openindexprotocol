use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};

use crate::Setup;

pub struct CreateTokenAccountTransaction {
    pub transaction: Transaction,
}
pub fn create_token_account_transaction(
    mint: &Pubkey,
    _setup: &Setup,
) -> CreateTokenAccountTransaction {
    let payer = &_setup.payer;
    let instruction =
        create_associated_token_account(&payer.pubkey(), &payer.pubkey(), mint, &spl_token::ID);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        _setup.recent_blockhashes,
    );

    CreateTokenAccountTransaction { transaction }
}
