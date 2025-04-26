use solana_sdk::{
    address_lookup_table::instruction::{
        create_lookup_table, derive_lookup_table_address, extend_lookup_table,
    },
    hash::Hash,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};

use crate::error::TransactionBuilderError;

/// dev - You can add approximately 30 addresses in one transaction.
///     - Each lookup table can store up to 256 addresses
///     - https://solana.com/es/developers/courses/program-optimization/lookup-tables
pub fn create_lookup_table_transaction(
    payer: &Keypair,
    authority_address: Pubkey,
    recent_slot: u64,
    recent_blockhashes: Hash,
    addresses: Vec<Pubkey>,
) -> Result<Transaction, TransactionBuilderError> {
    // Each lookup table can store up to 256 addresses
    if (addresses.len() > 256) {
        // return err
    }

    let tx_address_max = 30;

    let (create_account_instruction, lookup_table_address) =
        create_lookup_table(authority_address, payer.pubkey(), recent_slot);

        

    let mut instructions = vec![create_account_instruction];

    let _extend_lookup_table = |addresses_chunk: Vec<Pubkey>| {
        extend_lookup_table(
            lookup_table_address,
            authority_address,
            Some(payer.pubkey()),
            addresses_chunk,
        )
    };

    if addresses.len() > tx_address_max {
        for addresses_chunk in addresses.chunks(tx_address_max) {
            instructions.push(_extend_lookup_table(addresses_chunk.to_vec()));
        }
    } else {
        instructions.push(_extend_lookup_table(addresses));
    }

    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhashes,
    );

    Ok(transaction)
}
