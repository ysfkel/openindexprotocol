use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Module {
    pub key: Pubkey,
    pub active: bool,
    pub initialized: bool,
    pub bump: u8,
}

impl Module {
    pub const LEN: usize = 32 + 1 + 1 + 1;

    pub fn new(key: Pubkey, active: bool, bump: u8) -> Self {
        Self {
            key,
            active,
            initialized: true,
            bump,
        }
    }

    pub fn activate(&mut self) {
        self.active = false;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

impl IsInitialized for Module {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod test {
    use solana_program::pubkey::Pubkey;
    use super::Module;

    #[test]
    fn test_len() {
        let c = Module::new(Pubkey::new_unique(), true,253);
        assert_eq!(borsh::to_vec(&c).unwrap().len(), Module::LEN);
    }
}
