use crate::{
    issuance::instruction::mint_index_instruction_with_dynamic_accounts,
    openindex::pda::{
        find_controller_address, find_index_address, find_index_mint_address,
        find_index_mint_authority_address, find_index_mints_data_address,
        find_module_signer_address, find_registered_module_address,
    },
};
use solana_sdk::{hash::Hash, transaction::Transaction};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, Signer},
};

pub fn mint_index_transaction(
    amount: u64,
    payer: &Keypair,
    issuance_program_id: Pubkey,
    open_index_program_id: Pubkey,
    index_id: u64,
    controller_id: u64,
    token_account: Pubkey,
    recent_blockhashes: Hash,
    mints: Vec<Pubkey>,
    token_accounts: Vec<Pubkey>,
) -> Transaction {
    let controller_account = find_controller_address(&open_index_program_id, controller_id).0;
    let index_account = find_index_address(&open_index_program_id, &controller_account, index_id).0;
    let module_signer_account = find_module_signer_address(&issuance_program_id).0;
    let registered_module_account =
        find_registered_module_address(&open_index_program_id, &module_signer_account).0;
    let mint_account =
        find_index_mint_address(&open_index_program_id, &controller_account, controller_id).0;

    let mint_authority_account =
        find_index_mint_authority_address(&open_index_program_id, &controller_account, index_id).0;

    let index_mints_data_account =
        find_index_mints_data_address(&open_index_program_id, &controller_account, index_id).0;
    let token_program_account = spl_token::ID;
    let instruction = mint_index_instruction_with_dynamic_accounts(
        payer.pubkey(),
        issuance_program_id,
        open_index_program_id,
        module_signer_account,
        registered_module_account,
        controller_account,
        mint_account,
        mint_authority_account,
        index_account,
        index_mints_data_account,
        token_account,
        token_program_account,
        mints,
        token_accounts,
        index_id,
        amount,
    );

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhashes,
    )
}
