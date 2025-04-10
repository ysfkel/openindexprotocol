use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::instructions::CreateIndex;
use crate::instructions::InitProtocol;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum Instruction {
    InitProtocol {
        max_components_per_index: u32,
    },
    InitController,
    CreateIndex {
        amounts: Vec<u64>,
        mints: Vec<Pubkey>,
    },
    AddAsset,
    RemoveAsset,
    Mint,
    Redeem,
}
