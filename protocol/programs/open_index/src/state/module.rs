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
        self.active = true;
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
    use super::*; 

    #[test]
    fn test_new() {
        let key = Pubkey::new_unique(); 
        let c = Module::new(key, true,253);
        assert_eq!(c.key, key);
        assert_eq!(c.active, true);
        assert_eq!(c.is_initialized(), true);
        assert_eq!(c.bump, 253);
    }

    #[test]
    fn test_len() {
        let c = Module::new(Pubkey::new_unique(), true,253);
        assert_eq!(borsh::to_vec(&c).unwrap().len(), Module::LEN);
    }

    #[test]
    fn test_initialized() {
        let c = Module::new(Pubkey::new_unique(), true,253);
        assert_eq!(c.is_initialized(), true);
    }

    #[test]
    fn test_deactive() {
        let mut c = Module::new(Pubkey::new_unique(), true,253);
        c.deactivate();
        assert_eq!(c.active, false);
    }

    #[test]
    fn test_active() {
        let mut c = Module::new(Pubkey::new_unique(), false,253);
        c.activate();
        assert_eq!(c.active, true);
    }

}
