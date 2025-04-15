use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Component {
    pub uints: u64,
    pub mint: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
    pub initialized: bool,
}

impl IsInitialized for Component {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}
