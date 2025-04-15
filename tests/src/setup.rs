use {
    open_index::{entrypoint::process_instruction, state::Controller},
    open_index_lib::seeds::PROTOCOL_SEED,
    solana_program_test::{processor, BanksClient, ProgramTest},
    solana_sdk::{hash::Hash, pubkey::Pubkey, signature::Keypair},
};

// #[derive(Clone)]
pub struct Setup {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhashes: Hash,
    pub program_id: Pubkey,
}
pub async fn setup() -> Setup {
    let program_id = Pubkey::new_unique();
    let program_test = ProgramTest::new("open_index", program_id, processor!(process_instruction));
    let (mut banks_client, payer, recent_blockhashes) = program_test.start().await;

    Setup {
        banks_client,
        recent_blockhashes,
        payer,
        program_id,
    }
}
