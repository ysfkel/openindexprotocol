use crate::{get_protocol_pda, Setup};
use borsh::{BorshDeserialize, BorshSerialize};
use open_index_lib::{instruction::ProtocolInstruction , seeds::PROTOCOL_SEED};
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
    let (protocol_pda, _) = get_protocol_pda(program_id);

    let instruction  = ProtocolInstruction::init_protocol(program_id.clone(), payer.pubkey().clone(), protocol_pda.clone());
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
