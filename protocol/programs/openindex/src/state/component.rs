use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

use super::AccountType;

/// Component
///
/// Per-mint metadata for a *single* asset that makes up an index.  
/// Created by `AddIndexComponents`, referenced by `Mint` and `Redeem`.
///
/// One `Component` account exists for each `(index_mint, component_mint)`
/// pair and stores the fixed “recipe” amount (`units`) that backs **one**
/// index token.
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Component {
    /// Account type. It can be **Uninitialized** or **Component**.
    pub account_type: AccountType,

    /// Number of component units that back **one** index token.
    pub uints: u64,

    /// SPL mint address of the component asset.
    pub mint: Pubkey,

    /// PDA bump seed for `component_account`.
    pub bump: u8,

    /// PDA bump seed for the component’s vault account.
    pub vault_bump: u8,

    /// Set to `true` by `AddIndexComponents`; queried via `IsInitialized`.
    initialized: bool,
}

impl Component {
    /// Packed size in bytes:
    /// * 1  – `account_type`
    /// * 8  – `units`
    /// * 32 – `mint`
    /// * 1  – `bump`
    /// * 1  – `vault_bump`
    /// * 1  – `initialized`
    pub const LEN: usize = 1 + 8 + 32 + 1 + 1 + 1;

   /// Constructor used by `process_add_index_components`.
    pub fn new(uints: u64, mint: Pubkey, bump: u8, vault_bump: u8) -> Self {
        Self {
            account_type: AccountType::Component,
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
