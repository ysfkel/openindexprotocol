use solana_sdk::hash::Hash;
use solana_sdk::rent::Rent;
use solana_sdk::signature::Signer;
use solana_sdk::{
    program_pack::Pack, signature::Keypair, system_instruction::create_account,
    transaction::Transaction,
};
use spl_token::{instruction::initialize_mint, state::Mint};

pub fn create_mint_acccount_transaction(
    payer: &Keypair,
    mint: &Keypair,
    recent_blockhashes: Hash,
    rent: &Rent,
) -> Transaction {
    let mint_space = Mint::LEN;
    let lamports = rent.minimum_balance(mint_space);

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

    Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&payer.pubkey()),
        &[payer, mint],
        recent_blockhashes,
    )
}
