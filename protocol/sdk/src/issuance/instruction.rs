use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum IssuanceInstruction {
    InitConfig,
    Issue,
    Redeem,

    RegisterHooks { hooks: Vec<Pubkey> },
    UnregisterHooks { hooks: Vec<Pubkey> },
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum ExecuteHookInstruction {
    Execute { index_id: u64, amount: u64 }, // Discriminator 0
}
