use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Protocol {
    pub owner: Pubkey,
    pub next_controller_id: u64,
    pub initialized: bool,
    pub bump: u8,
}

impl Protocol {
    pub const LEN: usize = 32 + 8 + 1 + 1;
    pub fn new(owner: Pubkey, bump: u8) -> Self {
        Self {
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
    use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};
    use super::Protocol;

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
