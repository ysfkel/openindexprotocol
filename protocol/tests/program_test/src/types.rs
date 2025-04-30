use solana_program_test::BanksClientError;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

pub type BanksClientResult = Result<(), BanksClientError>;

pub struct ProcessInitProtocolResult {
    pub result: BanksClientResult,
}

pub struct ProcessInitModuleResult {
    pub registered_module_pda: Pubkey,
    pub module_signer_pda: Pubkey,
    pub result: BanksClientResult,
}

pub struct ProcessAddIndexComponentsResult {
    pub index_id: u64,
    pub controller_id: u64,
    pub mints: Vec<Pubkey>,
    pub amounts: Vec<u64>,
    pub result: BanksClientResult,
}

pub struct ProcessControllerGlobalConfigResult {
    pub result: BanksClientResult,
}

pub struct ProcessInitControllerResult {
    pub controller_id: u64,
    pub controller_pda: Pubkey,
    pub result: BanksClientResult,
}

pub struct ProcessCreateIndexResult {
    pub result: BanksClientResult,
    pub controller_pda: Pubkey,
    pub index_id: u64,
}
