use crate::{setup, Setup};
use solana_sdk::signature::Signer;
use solana_sdk::{
    program_pack::Pack, signature::Keypair, system_instruction::create_account,
    transaction::Transaction,
};
use spl_token::{instruction::initialize_mint, state::Mint};

pub struct CreateMintAccountTransaction {
    pub transaction: Transaction,
}
pub async fn create_mint_acccount_transaction(
    mint: &Keypair,
    _setup: &Setup,
) -> CreateMintAccountTransaction {
    let payer = &_setup.payer;
    let recent_blockhashes = &_setup.recent_blockhashes;

    let mint_space = Mint::LEN;
    let lamports = _setup.rent.minimum_balance(mint_space);

    let create_account_instruction = create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        lamports,
        mint_space as u64,
        &spl_token::id(),
    );

    let initialize_mint_instruction = initialize_mint(
        &spl_token::id(),
        &mint.pubkey(),
        &payer.pubkey(),
        Some(&payer.pubkey()),
        9,
    )
    .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&payer.pubkey()),
        &[&payer, mint],
        recent_blockhashes.clone(),
    );
    CreateMintAccountTransaction { transaction }
}
