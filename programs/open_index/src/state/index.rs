use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Index {
    pub id: u64,
    pub owner: Pubkey,
    pub manager: Pubkey,
    pub initialized: bool,
    pub bump: u8,
    pub mints: Vec<Pubkey>,
}
// PDA(["index", index_id.to_le_bytes()]) // ✅ globally unique

impl Index {
    pub fn new( 
        id: u64,
        owner: Pubkey,
        manager: Pubkey,
        mints: Vec<Pubkey>,
        bump: u8,
    ) -> Self {
        Self {
            id,
            owner,
            manager,
            initialized: true,
            bump,
            mints,
        }
    }
    pub fn len(mints_len: usize) -> usize {
        // 8 bytes for u64 (id)
        // 32 bytes for owner Pubkey
        // 32 bytes for manager Pubkey
        // 1 byte for bool
        // 1 byte for bump
        // 4 bytes for Vec length (Borsh/Anchor use u32)
        // 32 * n for each Pubkey in mints
        8 + 32 + 32 + 1 + 1 + 4 + (mints_len * 32)
    }
}

impl IsInitialized for Index {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}
