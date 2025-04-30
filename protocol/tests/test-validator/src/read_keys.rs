use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};
use std::{
    env,
    path::{Path, PathBuf},
};

fn get_workspace_root() -> PathBuf {
    let mut path = env::current_dir().expect("failed to get current dir");
    let parent = path.parent().expect("Failed to get parent dir");
    let workspace_root = parent
        .parent()
        .and_then(|p| p.parent())
        .expect("Failed to get workspace root");

    workspace_root.to_path_buf()
}

pub fn get_payer_keypair() -> Keypair {
    let payer_path = env::var("HOME").expect("Failed to get HOME env var");
    read_keypair_file(format!("{}/.config/solana/id.json", payer_path))
        .expect("Failed to read payer keypair")
}

pub fn get_openindex_keypair() -> Keypair {
    let keypair_path = get_workspace_root().join("target/deploy/openindex-keypair.json");
    read_keypair_file(&keypair_path).expect("Failed to read openindex keypair")
}

pub fn get_issuance_keypair() -> Keypair {
    let keypair_path = get_workspace_root().join("target/deploy/issuance-keypair.json");
    read_keypair_file(&keypair_path).expect("Failed to read issuance program keypair")
}

pub fn get_openindex_program_id() -> Pubkey {
    let k = get_openindex_keypair();
    k.pubkey()
}
pub fn get_issuance_program_id() -> Pubkey {
    let k = get_issuance_keypair();
    k.pubkey()
}
