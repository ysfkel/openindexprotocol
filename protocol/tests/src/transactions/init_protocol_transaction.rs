use crate::Setup;
use borsh::{BorshDeserialize, BorshSerialize};
use open_index_lib::{
    instruction::{init_protocol_instruction, ProtocolInstruction},
    pda::find_protocol_address,
    seeds::PROTOCOL_SEED,
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    system_program,
    transaction::Transaction,
};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub struct InitProtocolTransaction {
    pub protocol_pda: Pubkey,
    pub transaction: Transaction,
}

pub fn init_protocol_transaction(_setup: &Setup) -> InitProtocolTransaction {
    let payer = &_setup.payer;
    let program_id = &_setup.program_id;
    let recent_blockhashes = &_setup.recent_blockhashes;
    let (protocol_pda, _) = find_protocol_address(program_id);

    let instruction = init_protocol_instruction(
        program_id.clone(),
        payer.pubkey().clone(),
        protocol_pda.clone(),
    );
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        *recent_blockhashes,
    );

    InitProtocolTransaction {
        protocol_pda,
        transaction,
    }
}
