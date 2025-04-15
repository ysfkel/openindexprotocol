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
    use solana_program::pubkey::Pubkey;
    use super::Protocol;

    #[test]
    fn test_len() {
        let c = Protocol::new(Pubkey::new_unique(), 253);
        assert_eq!(borsh::to_vec(&c).unwrap().len(), Protocol::LEN);
    }
}
