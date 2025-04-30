use openindex_sdk::openindex::{
    instruction::init_protocol_instruction,
    pda::{find_module_signer_address, find_registered_module_address},
    transaction::init_module_transaction,
};
use solana_sdk::pubkey::Pubkey;

use crate::{BanksClientResult, ProcessInitModuleResult, Setup};

pub async fn process_init_module(
    module_program_id: Pubkey,
    _setup: &Setup,
) -> ProcessInitModuleResult {
    let transaction = init_module_transaction(
        &_setup.payer,
        _setup.program_id,
        module_program_id,
        _setup.recent_blockhashes,
    );

    let result = _setup
        .banks_client
        .process_transaction(transaction.clone())
        .await;

    let module_signer_pda = find_module_signer_address(&module_program_id).0;
    let registered_module_pda =
        find_registered_module_address(&_setup.program_id, &module_signer_pda).0;

    ProcessInitModuleResult {
        registered_module_pda,
        module_signer_pda,
        result,
    }
}
