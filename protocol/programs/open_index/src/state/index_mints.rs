use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_pack::IsInitialized, pubkey::Pubkey};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct IndexMints {
    pub mints: Vec<Pubkey>,
    pub initialized: bool,
    pub bump: u8,
}

impl IndexMints {
    pub fn new(mints: Vec<Pubkey>, bump: u8) -> Self {
        Self {
            mints,
            initialized: true,
            bump,
        }
    }
    pub fn calc_len(mints_len: usize) -> usize {
        4 + (mints_len * 32) + 1 + 1
    }

    pub fn len(&self) -> usize {
        4 + (self.mints.len() * 32) + 1 + 1
    }

    
}

impl IsInitialized for IndexMints {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod test {
    use solana_program::pubkey::Pubkey;

    use super::IndexMints;

    #[test]
    fn test_len() {
        let c = IndexMints::new(vec![Pubkey::new_unique(),Pubkey::new_unique()],254);
        assert_eq!(borsh::to_vec(&c).unwrap().len(), IndexMints::calc_len(2));
        assert_eq!(borsh::to_vec(&c).unwrap().len(), c.len());
    }
}
