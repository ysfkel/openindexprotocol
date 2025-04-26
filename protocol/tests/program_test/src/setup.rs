use std::ops::{Add, Mul};

use {
    solana_address_lookup_table_program::processor::Entrypoint, solana_program_test::{processor, BanksClient, ProgramTest}, solana_sdk::{hash::Hash, pubkey::Pubkey, rent::Rent, signature::Keypair, sysvar::{Sysvar, SysvarId}}
};

pub struct Setup {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhashes: Hash,
    pub program_id: Pubkey,
    pub issuance_program_id: Pubkey,
    pub rent: Rent,
}

// pub fn alt_process_instruction(
//     _program_id: &Pubkey,
//     _accounts: &[AccountInfo],
//     _instruction_data: &[u8],
// ) -> ProgramResult {
//     Ok(())
// }

use solana_program::log::sol_log_compute_units;
use solana_program_test::EbpfVm;
use solana_program_test::InvokeContext;

// pub fn alt_dummy_processor(
//     _vm: *mut EbpfVm<InvokeContext<'static>>,
//     _arg1: u64,
//     _arg2: u64,
//     _arg3: u64,
//     _arg4: u64,
//     _arg5: u64,
// ) {
//     // Do nothing.
//       // Log a message; this might help simulate compute unit consumption.
//       solana_program::msg!("ALT dummy processor executed");

//       // Dummy loop to consume compute units.
//     // This arithmetic loop is intended to use some cycles.
//     let mut dummy: u64 = 1;
//     for i in 0..100{
//         dummy +=i; 
//     }
//     // Prevent compiler from optimizing away the loop.
  

//     // Log the remaining compute units.
//     sol_log_compute_units();
// }

pub async fn setup() -> Setup {
    let program_id = Pubkey::new_unique();
    let issuance_program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "open_index",
        program_id,
        processor!(open_index::entrypoint::process_instruction),
    );

    // program_test.add_builtin_program(
    //     "solana_address_lookup_table_program",
    //     solana_program::address_lookup_table::program::ID,
    //     solana_address_lookup_table_program::processor::
    // );
    
 
    // program_test.add_builtin_program(
    //     "solana_address_lookup_table_program",
    //     solana_program::address_lookup_table::program::ID,
    //     alt_dummy_processor,
    // );

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
