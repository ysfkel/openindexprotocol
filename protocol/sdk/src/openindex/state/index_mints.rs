use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

use super::AccountType;

/// IndexMints
///
/// Variable-length PDA that stores the **ordered list of component SPL
/// mints** backing a given index.  
/// Created by `AddIndexComponents`, read by `Mint` and `Redeem`.
/// 
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct IndexMints {
    /// Account type. It can be **Uninitialized** or **IndexMints**.
    pub account_type: AccountType,
    
    /// Ordered list of component mint addresses.
    pub mints: Vec<Pubkey>,

    /// Set to `true` by `AddIndexComponents`; queried via `IsInitialized`.
    pub initialized: bool,

    /// PDA bump seed for `index_mints_account`.
    pub bump: u8,
}

impl IndexMints {
    /// Constructor used by `process_add_index_components`.
    pub fn new(mints: Vec<Pubkey>, bump: u8) -> Self {
        Self {
            account_type: AccountType::IndexMints,
            mints,
            initialized: true,
            bump,
        }
    }

    /// Compute the packed size **before** the account is created.
    ///
    /// Layout:  
    /// * 1  – `account_type`  
    /// * 4  – `Vec` length prefix (`u32`)  
    /// * N×32 – each `Pubkey` in `mints`  
    /// * 1  – `initialized`  
    /// * 1  – `bump`
    pub fn calc_len(mints_len: usize) -> usize {
        1 + 4 + (mints_len * 32) + 1 + 1
    }

    /// Compute the packed size from an existing instance.
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
