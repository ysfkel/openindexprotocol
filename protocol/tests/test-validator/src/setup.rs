use std::sync::Arc;

use crate::{get_issuance_program_id, get_open_index_program_id, get_payer_keypair};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair};

pub struct TestContext {
    pub payer: Keypair,
    pub open_index_program_id: Pubkey,
    pub issuance_program_id: Pubkey,
    pub client: Arc<RpcClient>,
}

pub fn setup() -> TestContext {
    let rpc_url = "http://127.0.0.1:8899";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    TestContext {
        payer: get_payer_keypair(),
        open_index_program_id: get_open_index_program_id(),
        issuance_program_id: get_issuance_program_id(),
        client: Arc::new(client),
    }
}
