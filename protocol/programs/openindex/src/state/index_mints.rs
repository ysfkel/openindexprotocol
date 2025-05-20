use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

use super::AccountType;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct IndexMints {
    pub account_type: AccountType,
    pub mints: Vec<Pubkey>,
    pub initialized: bool,
    pub bump: u8,
}

impl IndexMints {
    pub fn new(mints: Vec<Pubkey>, bump: u8) -> Self {
        Self {
            account_type: AccountType::IndexMints,
            mints,
            initialized: true,
            bump,
        }
    }
    pub fn calc_len(mints_len: usize) -> usize {
        1 + 4 + (mints_len * 32) + 1 + 1
    }

    pub fn len(&self) -> usize {
        1 + 4 + (self.mints.len() * 32) + 1 + 1
    }
}

impl IsInitialized for IndexMints {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let owner = Pubkey::new_unique();
        let c = IndexMints::new(vec![Pubkey::new_unique(), Pubkey::new_unique()], 254);
        assert_eq!(c.mints.len(), 2);
        assert_eq!(c.is_initialized(), true);
        assert_eq!(c.bump, 254);
    }

    #[test]
    fn test_len() {
        let c = IndexMints::new(vec![Pubkey::new_unique(), Pubkey::new_unique()], 254);
        assert_eq!(borsh::to_vec(&c).unwrap().len(), IndexMints::calc_len(2));
        assert_eq!(borsh::to_vec(&c).unwrap().len(), c.len());
    }
}
