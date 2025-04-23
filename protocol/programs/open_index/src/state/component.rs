use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

/// A Conponent of Index token
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Component {
    pub uints: u64,
    /// The token mint of the component
    pub mint: Pubkey,
    /// Component PDA bump
    pub bump: u8,
    /// The PDA bump of the vault account of the component
    pub vault_bump: u8,
    /// Component initialzed state - set true when component is initialized
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
    use super::*;

    #[test]
    fn test_new() {
        let mint = Pubkey::new_unique();
        let c = Component::new(1, mint, 253, 252);
        assert_eq!(c.uints, 1);
        assert_eq!(c.mint, mint);
        assert_eq!(c.bump, 253);
        assert_eq!(c.vault_bump, 252);
        assert_eq!(c.initialized, true);
    }

    #[test]
    fn test_len() {
        let c = Component::new(1, Pubkey::new_unique(), 1, 1);
        assert_eq!(borsh::to_vec(&c).unwrap().len(), Component::LEN);
    }
}
