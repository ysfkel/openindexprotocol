use std::sync::Arc;

use crate::{get_issuance_program_id, get_openindex_program_id, get_payer_keypair};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, rent::Rent, signature::Keypair,
    sysvar::SysvarId,
};

pub struct TestContext {
    pub payer: Keypair,
    pub openindex_program_id: Pubkey,
    pub issuance_program_id: Pubkey,
    pub client: RpcClient,
    pub rent: Rent,
}

pub fn setup() -> TestContext {
    let rpc_url = "http://127.0.0.1:8899";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let rent_account = client
        .get_account(&Rent::id())
        .expect("failed to get rent account");
    let rent = rent_account.deserialize_data().unwrap();
    TestContext {
        payer: get_payer_keypair(),
        openindex_program_id: get_openindex_program_id(),
        issuance_program_id: get_issuance_program_id(),
        client,
        rent,
    }
}
