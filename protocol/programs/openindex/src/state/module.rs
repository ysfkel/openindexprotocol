use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{example_mocks::solana_account::Account, program_pack::IsInitialized, pubkey::Pubkey};

use super::AccountType;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Module {
    pub account_type: AccountType,
    pub is_active: bool,
    pub initialized: bool,
    pub bump: u8,
}

impl Module {
    pub const LEN: usize = 1 + 1 + 1 + 1;

    pub fn new(is_active: bool, bump: u8) -> Self {
        Self {
            account_type: AccountType::Module,
            is_active,
            initialized: true,
            bump,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

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
