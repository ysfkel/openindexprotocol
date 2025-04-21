use core::num;

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
use open_index_lib::seeds::PROTOCOL_SEED;
use solana_program_test::BanksClientError;
use solana_sdk::{
    clock::sysvar,
    instruction::InstructionError,
    msg,
    program_pack::Pack,
    rent::Rent,
    signature::Keypair,
    syscalls,
    system_instruction::create_account,
    sysvar::Sysvar,
    transaction::{Transaction, TransactionError},
};
use spl_token::{instruction::initialize_mint, state::Mint};
use {solana_program::pubkey::Pubkey, solana_program_test::tokio, solana_sdk::signature::Signer};

pub struct InitAccountTransaction {
    pub transaction: Transaction,
}

async fn create_acccount_transaction(
    account: Keypair,
    len: usize,
    lamports: u64,
    Setup {
        banks_client,
        recent_blockhashes,
        payer,
        program_id,
        rent,
    }: &Setup,
) -> InitAccountTransaction {
    let _setup: Setup = setup().await;

    let create_account_instruction = create_account(
        &payer.pubkey(),
        &account.pubkey(),
        lamports,
        lamports,
        &spl_token::id(),
    );

    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction],
        Some(&payer.pubkey()),
        &[&payer, &account],
        recent_blockhashes.clone(),
    );

    InitAccountTransaction { transaction }
}
