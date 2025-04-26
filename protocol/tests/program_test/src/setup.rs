use std::ops::{Add, Mul};

use {
    solana_address_lookup_table_program::processor::Entrypoint,
    solana_program_test::{processor, BanksClient, ProgramTest},
    solana_sdk::{
        hash::Hash,
        pubkey::Pubkey,
        rent::Rent,
        signature::Keypair,
        sysvar::{Sysvar, SysvarId},
    },
};

pub struct Setup {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhashes: Hash,
    pub program_id: Pubkey,
    pub issuance_program_id: Pubkey,
    pub rent: Rent,
}

pub async fn setup() -> Setup {
    let program_id = Pubkey::new_unique();
    let issuance_program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "open_index",
        program_id,
        processor!(open_index::entrypoint::process_instruction),
    );

   
    program_test.add_program(
        "issuance",
        issuance_program_id,
        processor!(issuance::entrypoint::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhashes) = program_test.start().await;
    // get rent
    let rent_account = banks_client
        .get_account(Rent::id())
        .await
        .expect("RPC error")
        .unwrap();

    let rent: Rent = rent_account.deserialize_data().unwrap();

    Setup {
        banks_client,
        recent_blockhashes,
        payer,
        program_id,
        issuance_program_id,
        rent,
    }
}
