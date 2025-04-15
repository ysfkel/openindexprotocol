use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_pack::IsInitialized;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct ControllerGlobalConfig {
    pub max_index_components: u32,
    pub initialized: bool,
    pub bump: u8,
}

impl ControllerGlobalConfig {
    pub const LEN: usize = 4 + 1 + 1;

    pub fn new(max_index_components: u32, bump: u8) -> Self {
        Self {
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
