use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Component {
    pub uints: u64,
    pub mint: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
}

impl Component {
    pub const LEN: usize = 1 + 32;

    pub fn new(uints: u64, mint: Pubkey, bump: u8, vault_bump: u8) -> Self {
        Self {
            uints,
            mint,
            bump,
            vault_bump,
        }
    }
}
