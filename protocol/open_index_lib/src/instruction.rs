use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum Instruction {
    InitProtocol,
    InitController,

    InitControllerGlobalConfig {
        max_index_components: u32,
    },
    InitModule,
    CreateIndex {
        amounts: Vec<u64>,
        mints: Vec<Pubkey>,
    },
    Mint {
        index_id: u64,
        amount: u64,
    },
    Redeem,
    //..
}
