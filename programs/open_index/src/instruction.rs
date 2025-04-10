use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::instructions::CreateIndex;
use crate::instructions::InitProtocol;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum Instruction {
    InitProtocol {
        max_components_per_index: u32,
    },
    InitController,
    CreateIndex {
        amounts: Vec<u64>,
        mints: Vec<Pubkey>,
    },
    AddAsset,
    RemoveAsset,
    Mint,
    Redeem,
}

#[cfg(test)]
mod test {
    use borsh::BorshSerialize;

    use super::*;

    #[test]
    fn test_deserialize() {
        let k1 = Pubkey::new_unique();
        let k2 = Pubkey::new_unique();
        let mints = vec![k1, k2];
        let amounts = vec![1, 2];
        let amounts2 = vec![1, 3];

        let payload = CreateIndexPayload {
            mints: mints.clone(),
            amounts: amounts.clone(),
        };

        let mut serialized = Vec::new();
        payload
            .serialize(&mut serialized)
            .expect("Failed to serialize");

        let mut data = vec![0];
        data.extend_from_slice(&serialized);

        let instruction = Instruction::unpack(&data).unwrap();

        assert_eq!(instruction, Instruction::CreateIndex { mints, amounts });
    }
}
