use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_pack::IsInitialized;

use super::AccountType;

/// ControllerGlobalConfig
///
/// Singleton account that stores *controller-wide limits and guards*
/// enforced uniformly across every controller and index in the protocol.
///
/// Currently it holds only `max_index_components`, but you can extend it
/// with fee rates, rebalancing cool-downs, etc.  
/// Created once by `InitControllerGlobalConfig`.
///
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct ControllerGlobalConfig {
    /// Account type. It can be **Uninitialized** or **ControllerGlobalConfig**.
    pub account_type: AccountType,

    /// Hard cap on how many component mints an index may contain.
    pub max_index_components: u32,

    /// Set to `true` by `InitControllerGlobalConfig`; queried via `IsInitialized`.
    pub initialized: bool,

    /// PDA bump seed for `controller_global_config_account`.
    pub bump: u8,
}

impl ControllerGlobalConfig {
    /// Packed size in bytes:
    /// * 1 – `account_type`
    /// * 4 – `max_index_components`
    /// * 1 – `initialized`
    /// * 1 – `bump`
    pub const LEN: usize = 1 + 4 + 1 + 1;

    /// Constructor used by the processor.
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
