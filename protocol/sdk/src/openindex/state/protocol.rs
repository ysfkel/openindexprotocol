use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

use super::AccountType;

/// Protocol
///
/// Top-level state account for the entire Open-Index program.  
/// Only **one** instance of `Protocol` exists (PDA seed = `b"protocol"`),
/// created by the `InitProtocol` instruction.
///
/// It stores:
/// * the protocol-governance authority (`owner`);
/// * a monotonic counter for assigning **controller IDs**;
/// * a bump seed so the PDA can sign future CPIs.
/// 
#[derive(BorshDeserialize, BorshSerialize, Debug, Default)]
pub struct Protocol {

    /// Account type. It can be Uninitialized, Protocol
    pub account_type: AccountType,

    /// Governance authority that can register modules, change fees,
    /// and transfer ownership.
    pub owner: Pubkey,

    /// Auto-incrementing ID given to each new controller when
    /// `InitController` is executed. Starts at **1**.
    pub next_controller_id: u64,

    /// Flag set to `true` by `InitProtocol`; queried by
    /// `IsInitialized` trait.
    pub initialized: bool,

    /// PDA bump seed for `protocol_account`.
    pub bump: u8,
}

impl Protocol {

    /// Packed size in bytes.  
    ///   1  – `account_type` (u8)  
    /// + 32 – `owner` (Pubkey)  
    /// + 8  – `next_controller_id` (u64)  
    /// + 1  – `initialized` (bool as u8)  
    /// + 1  – `bump` (u8)
    pub const LEN: usize = 1 + 32 + 8 + 1 + 1;

    /// Constructor used by `process_init_protocol`.
    pub fn new(owner: Pubkey, bump: u8) -> Self {
        Self {
            account_type: AccountType::Protocol,
            owner,
            bump,
            initialized: true,
            next_controller_id: 1,
        }
    }

    pub fn generate_next_controller_id(&mut self) {
        self.next_controller_id += 1;
    }

    pub fn get_next_controller_id(&self) -> u64 {
        self.next_controller_id
    }
}

impl IsInitialized for Protocol {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod test {
    use super::Protocol;
    use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

    #[test]
    fn test_new() {
        let owner = Pubkey::new_unique();
        let c = Protocol::new(owner, 253);
        assert_eq!(c.owner, owner);
        assert_eq!(c.is_initialized(), true);
        assert_eq!(c.bump, 253);
    }

    #[test]
    fn test_len() {
        let c = Protocol::new(Pubkey::new_unique(), 253);
        assert_eq!(borsh::to_vec(&c).unwrap().len(), Protocol::LEN);
    }

    #[test]
    fn test_initialized() {
        let c = Protocol::new(Pubkey::new_unique(), 253);
        assert_eq!(c.is_initialized(), true);
    }

    #[test]
    fn test_next_controller_id() {
        let c = Protocol::new(Pubkey::new_unique(), 253);
        assert_eq!(c.next_controller_id, 1);
    }
}
