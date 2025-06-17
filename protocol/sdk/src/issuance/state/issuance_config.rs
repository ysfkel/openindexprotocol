use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

use super::AccountType;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct IssuanceConfig {
    pub account_type: AccountType,
    pub allowed_hooks: Vec<Pubkey>,
    pub openindex_program_id: Pubkey,
    pub initialized: bool,
    pub bump: u8,
}

impl IssuanceConfig {
    pub fn new(openindex_program_id: Pubkey, allowed_hooks: Vec<Pubkey>, bump: u8) -> Self {
        Self {
            account_type: AccountType::IssuanceConfig,
            allowed_hooks,
            openindex_program_id,
            initialized: true,
            bump,
        }
    }

    /// Compute the packed size **before** the account is created.
    ///
    /// Layout:  
    /// * 1  – `account_type`  
    /// * 4  – `Vec` length prefix (`u32`)  
    /// * N×32 – each `Pubkey` in `allowed_hooks`  
    /// * 1  – `initialized`  
    /// * 1  – `bump`
    pub fn calc_len(allowed_hooks_len: usize) -> usize {
        1 + 4 + (allowed_hooks_len * 32) + 32 + 1 + 1
    }

    /// Compute the packed size from an existing instance.
    pub fn len(&self) -> usize {
        1 + 4 + (self.allowed_hooks.len() * 32) + 32 + 1 + 1
    }
}

impl IsInitialized for IssuanceConfig {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let c = IssuanceConfig::new(Pubkey::new_unique(),vec![Pubkey::new_unique(), Pubkey::new_unique()], 254);
        assert_eq!(c.allowed_hooks.len(), 2);
        assert_eq!(c.is_initialized(), true);
        assert_eq!(c.bump, 254);
    }

    #[test]
    fn test_len() {
        let c = IssuanceConfig::new(Pubkey::new_unique(),vec![Pubkey::new_unique(), Pubkey::new_unique()], 254);
        assert_eq!(
            borsh::to_vec(&c).unwrap().len(),
            IssuanceConfig::calc_len(2)
        );
        assert_eq!(borsh::to_vec(&c).unwrap().len(), c.len());
    }
}
