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
