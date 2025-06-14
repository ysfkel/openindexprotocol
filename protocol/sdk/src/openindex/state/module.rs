use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_pack::IsInitialized;

use super::AccountType;

/// Module
///
/// Metadata account for an **external program** that has been registered
/// with the protocol (see `InitModule`).  
///
/// A module is allowed to CPI into the core program to perform automated
/// tasks such as rebalancing, fee routing, or strategy execution.
///
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Module {
    /// Account type. It can be **Uninitialized** or **Module**.
    pub account_type: AccountType,

    /// If `false`, the module is paused and CPIs from it should be rejected.
    pub is_active: bool,

    /// Set to `true` by `InitModule`; queried via `IsInitialized`.
    pub initialized: bool,

    /// PDA bump seed for `registered_module_account`.
    pub bump: u8,
}

impl Module {
    /// Packed size in bytes:
    /// * 1 – `account_type`
    /// * 1 – `is_active`
    /// * 1 – `initialized`
    /// * 1 – `bump`
    pub const LEN: usize = 1 + 1 + 1 + 1;

    pub fn new(is_active: bool, bump: u8) -> Self {
        Self {
            account_type: AccountType::Module,
            is_active,
            initialized: true,
            bump,
        }
    }
    /// Activate the module (e.g. after governance vote).
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Deactivate / pause the module.
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Convenience getter.
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

impl IsInitialized for Module {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let c = Module::new(true, 253);
        assert_eq!(c.is_active(), true);
        assert_eq!(c.is_initialized(), true);
        assert_eq!(c.bump, 253);
    }

    #[test]
    fn test_len() {
        let c = Module::new(true, 253);
        assert_eq!(borsh::to_vec(&c).unwrap().len(), Module::LEN);
    }

    #[test]
    fn test_initialized() {
        let c = Module::new(true, 253);
        assert_eq!(c.is_initialized(), true);
    }

    #[test]
    fn test_deactive() {
        let mut c = Module::new(true, 253);
        c.deactivate();
        assert_eq!(c.is_active, false);
    }

    #[test]
    fn test_active() {
        let mut c = Module::new(false, 253);
        c.activate();
        assert_eq!(c.is_active, true);
    }
}
