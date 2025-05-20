use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_pack::IsInitialized;

use super::AccountType;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct ControllerGlobalConfig {
    pub account_type: AccountType,
    pub max_index_components: u32,
    pub initialized: bool,
    pub bump: u8,
}

impl ControllerGlobalConfig {
    pub const LEN: usize = 1 + 4 + 1 + 1;

    pub fn new(max_index_components: u32, bump: u8) -> Self {
        Self {
            account_type: AccountType::ControllerGlobalConfig,
            max_index_components,
            initialized: true,
            bump,
        }
    }
}

impl IsInitialized for ControllerGlobalConfig {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let c = ControllerGlobalConfig::new(50, 254);
        assert_eq!(c.max_index_components, 50);
        assert_eq!(c.bump, 254);
        assert_eq!(c.initialized, true);
    }

    #[test]
    fn test_len() {
        let c = ControllerGlobalConfig::new(50, 254);
        assert_eq!(
            borsh::to_vec(&c).unwrap().len(),
            ControllerGlobalConfig::LEN
        );
    }
}
