use crate::openindex::{
    instruction::add_index_components_instruction,
    pda::{
        find_controller_address, find_controller_global_config_address, find_index_address,
        find_index_mints_data_address,
    },
};
use solana_sdk::{
    hash::Hash,
    message::{v0::Message as V0Message, AddressLookupTableAccount, VersionedMessage},
    transaction::VersionedTransaction,
};

use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

/// Creates a transaction to add index components
pub fn add_index_components_versioned_transaction(
    payer: &Keypair,
    program_id: Pubkey,
    index_id: u64,
    controller_id: u64,
    recent_blockhashes: Hash,
    mints: Vec<Pubkey>,
    amounts: Vec<u64>,
    lookup_table_account: AddressLookupTableAccount,
) -> VersionedTransaction {
    let controller_pda = find_controller_address(&program_id, controller_id).0;
    let (index_pda, _) = find_index_address(&program_id, &controller_pda, index_id);
    let (controller_global, _) = find_controller_global_config_address(&program_id);
    let (index_mints_data_pda, _) =
        find_index_mints_data_address(&program_id, &controller_pda, index_id);

    let instruction = add_index_components_instruction(
        program_id.clone(),
        payer.pubkey().clone(),
        index_pda,
        index_mints_data_pda,
        controller_pda,
        controller_global,
        mints,
        amounts,
    );

    let v0_msg: V0Message = V0Message::try_compile(
        &payer.pubkey(), // fee-payer
        &[instruction],
        &[lookup_table_account.clone()],
        recent_blockhashes,
    )
    .unwrap();

    let versioned = VersionedMessage::V0(v0_msg);

    let versioned_tx = VersionedTransaction::try_new(
        versioned,
        &[payer], // only real signers (PDAs never sign)
    )
    .unwrap();

    versioned_tx
}
