use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

use super::AccountType;

/// Controller
///
/// Administrative domain that owns a *set of indexes*.  A protocol can
/// create many controllers (one per product line, DAO, or governance
/// realm).  Each controller keeps its own counter for index IDs and has
/// its own owner who can:
/// * create new indexes,
/// * transfer controller ownership,
/// * register a manager for individual indexes.
///
/// Created by the `InitController` instruction.

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Controller {
    /// Account type. It can be **Uninitialized** or **Controller**.
    pub account_type: AccountType,

    /// Monotonic identifier assigned by the protocol (starts at 1).
    pub id: u64,

    /// Authority that can create indexes and transfer ownership.
    pub owner: Pubkey,

    /// Auto-incrementing ID for the next index created under this controller.
    pub next_index_id: u64,

    /// Set to `true` by `InitController`; queried via `IsInitialized`.
    pub initialized: bool,

    /// PDA bump seed for `controller_account`.
    pub bump: u8,
}

impl Controller {
    /// Packed size in bytes:
    /// * 1  – `account_type`
    /// * 8  – `id`
    /// * 32 – `owner`
    /// * 8  – `next_index_id`
    /// * 1  – `initialized`
    /// * 1  – `bump`
    pub const LEN: usize = 1 + 8 + 32 + 8 + 1 + 1;

    /// Constructor used by `process_init_controller`.
    pub fn new(id: u64, owner: Pubkey, bump: u8) -> Self {
        Self {
            account_type: AccountType::Controller,
            id,
            owner,
            bump,
            initialized: true,
            next_index_id: 1,
        }
    }

    /// Increment `next_index_id` after successfully creating an index.
    pub fn generate_next_index_id(&mut self) {
        self.next_index_id += 1;
    }

    /// Read-only helper for the upcoming index ID.
    pub fn get_next_index_id(&self) -> u64 {
        self.next_index_id
    }
}

impl IsInitialized for Controller {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_len() {
        let c = Controller::new(1, Pubkey::new_unique(), 254);
        assert_eq!(borsh::to_vec(&c).unwrap().len(), Controller::LEN);
    }

    #[test]
    fn test_new() {
        let owner = Pubkey::new_unique();
        let c = Controller::new(1, owner, 253);
        assert_eq!(c.id, 1);
        assert_eq!(c.owner, owner);
        assert_eq!(c.is_initialized(), true);
        assert_eq!(c.next_index_id, 1);
        assert_eq!(c.bump, 253);
    }

    #[test]
    fn test_get_next_controller_id() {
        let c = Controller::new(1, Pubkey::new_unique(), 1);
        assert_eq!(c.next_index_id, 1);
    }

    #[test]
    fn test_generate_next_controller_id() {
        let mut c = Controller::new(1, Pubkey::new_unique(), 1);
        assert_eq!(c.next_index_id, 1);
        c.generate_next_index_id();
        assert_eq!(c.next_index_id, 2);
        c.generate_next_index_id();
        assert_eq!(c.next_index_id, 3);
    }
}
