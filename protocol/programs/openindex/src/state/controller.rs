use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Controller {
    pub id: u64,
    pub owner: Pubkey,
    pub next_index_id: u64,
    pub initialized: bool,
    pub bump: u8,
}

impl Controller {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 1;

    pub fn new(id: u64, owner: Pubkey, bump: u8) -> Self {
        Self {
            id,
            owner,
            bump,
            initialized: true,
            next_index_id: 1,
        }
    }

    pub fn generate_next_index_id(&mut self) {
        self.next_index_id += 1;
    }

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
