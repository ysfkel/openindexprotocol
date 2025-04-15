use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct IndexMints {
    pub mints: Vec<Pubkey>,
    pub initialized: bool,
    pub bump: u8,
}

impl IndexMints {
    pub fn new(mints: Vec<Pubkey>, bump: u8) -> Self {
        Self {
            mints,
            initialized: true,
            bump,
        }
    }
    pub fn len(mints_len: usize) -> usize {
        // 4 bytes for Vec length (Borsh/Anchor use u32)
        4 + (mints_len * 32) + 1 + 1
    }
}

impl IsInitialized for IndexMints {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}
