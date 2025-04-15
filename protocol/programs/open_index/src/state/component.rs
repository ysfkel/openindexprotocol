use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Component {
    pub uints: u64,
    pub mint: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
    initialized: bool,
}

impl Component {
    pub const LEN: usize = 8 + 32 + 1 + 1 + 1;

    pub fn new(uints: u64, mint: Pubkey, bump: u8, vault_bump: u8) -> Self {
        Self {
            uints,
            mint,
            bump,
            vault_bump,
            initialized: true,
        }
    }
}

impl IsInitialized for Component {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod test {
    use solana_program::pubkey::Pubkey;

    use super::Component;

    #[test]
    fn test_len() {
        let c = Component::new(1, Pubkey::new_unique(), 1, 1);
        assert_eq!(borsh::to_vec(&c).unwrap().len(), Component::LEN);
    }
}
