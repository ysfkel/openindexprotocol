use borsh::BorshDeserialize;
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

#[derive(BorshDeserialize, Debug)]
pub struct IndexMints {
    pub mints: Vec<Pubkey>,
    pub initialized: bool,
    pub bump: u8,
}

impl IsInitialized for IndexMints {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}
