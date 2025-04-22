use crate::{
    get_controller_pda, get_protocol_pda, init_controller_transaction, init_protocol_transaction,
    setup, Setup,
};
use borsh::{BorshDeserialize, BorshSerialize};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use open_index::{
    error::ProtocolError,
    state::{Controller, Protocol},
};
use solana_sdk::{
    program_pack::Pack, signature::Keypair, system_instruction::create_account, transaction::Transaction
};
use spl_token::{instruction::initialize_mint, state::Mint};
use solana_sdk::signature::Signer;

pub struct CreateMintAccountTransaction {
    pub transaction: Transaction,
}
pub async fn create_mint_acccount_transaction(
    mint: &Keypair,
    Setup {
        banks_client,
        recent_blockhashes,
        payer,
        program_id,
        rent,
    }: &Setup,
) -> CreateMintAccountTransaction {
    let _setup: Setup = setup().await;
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
