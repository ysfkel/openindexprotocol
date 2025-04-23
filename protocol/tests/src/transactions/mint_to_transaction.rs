use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token::instruction::mint_to_checked;

use crate::Setup;

pub struct MintToTransaction {
    pub transaction: Transaction,
}
pub fn mint_to_transaction(
    amount: u64,
    mint: &Pubkey,
    token_account: &Pubkey,
    _setup: &Setup,
) -> MintToTransaction {
    let payer = &_setup.payer;
    let instruction =
        create_associated_token_account(&payer.pubkey(), &payer.pubkey(), mint, &spl_token::ID);

    let instruction: solana_sdk::instruction::Instruction = mint_to_checked(
        &spl_token::ID,
        mint,
        token_account,
        &payer.pubkey(),
        &[&payer.pubkey()],
        amount,
        9,
    )
    .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        _setup.recent_blockhashes,
    );

    MintToTransaction { transaction }
}
