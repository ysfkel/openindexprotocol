use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

use super::AccountType;

/// Index
///
/// Metadata account for a single *tokenized index* created under a
/// 
/// controller.  Every index has:
/// * a unique `id` (monotonic within its controller),
/// * an SPL `mint` (stored separately; PDA seed = `b"index_mint"`),
/// * an optional `manager` who can rebalance / change components,
/// * an owner (initially the controller owner) who can transfer
///   management rights or close the index.
///
/// Created by the `CreateIndex` instruction.
/// 
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Index {
    /// Account type. It can be **Uninitialized** or **Index**.
    pub account_type: AccountType,

    /// Monotonic identifier scoped to its controller.
    pub id: u64,

    /// Authority that can transfer ownership or close the index.
    pub owner: Pubkey,

    /// Delegate allowed to add components, rebalance, etc.
    pub manager: Pubkey,

    /// Set to `true` by `CreateIndex`; queried via `IsInitialized`.
    pub initialized: bool,

    /// PDA bump seed for `index_account`.
    pub bump: u8,
}

impl Index {
    /// Packed size in bytes:
    /// * 1  – `account_type`
    /// * 8  – `id`
    /// * 32 – `owner`
    /// * 32 – `manager`
    /// * 1  – `initialized`
    /// * 1  – `bump`
    pub const LEN: usize = 1 + 8 + 32 + 32 + 1 + 1;

    /// Constructor used by `process_create_index`.
    pub fn new(id: u64, owner: Pubkey, manager: Pubkey, bump: u8) -> Self {
        Self {
            account_type: AccountType::Index,
            id,
            owner,
            manager,
            initialized: true,
            bump,
        }
    }
}

impl IsInitialized for Index {
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
        let manager = Pubkey::new_unique();
        let c = Index::new(1, owner, manager, 253);
        assert_eq!(c.id, 1);
        assert_eq!(c.owner, owner);
        assert_eq!(c.manager, manager);
        assert_eq!(c.is_initialized(), true);
        assert_eq!(c.bump, 253);
    }

    #[test]
    fn test_len() {
        let c = Index::new(1, Pubkey::new_unique(), Pubkey::new_unique(), 253);
        assert_eq!(borsh::to_vec(&c).unwrap().len(), Index::LEN);
    }
}
