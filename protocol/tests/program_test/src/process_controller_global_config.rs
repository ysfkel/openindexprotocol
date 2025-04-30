use openindex_sdk::openindex::transaction::{
    init_controller_global_config_transaction, init_protocol_transaction,
};

use crate::{ProcessControllerGlobalConfigResult, Setup};

pub async fn process_controller_global_config(
    max_index_components: u32,
    _setup: &Setup,
) -> ProcessControllerGlobalConfigResult {
    let transaction = init_controller_global_config_transaction(
        &_setup.payer,
        _setup.program_id,
        max_index_components,
        _setup.recent_blockhashes,
    );

    let init_protocol_instruction =
        init_protocol_transaction(&_setup.payer, _setup.program_id, _setup.recent_blockhashes);

    let _ = _setup
        .banks_client
        .process_transaction(init_protocol_instruction.clone())
        .await;

    let result = _setup
        .banks_client
        .process_transaction(transaction.clone())
        .await;

    ProcessControllerGlobalConfigResult { result }
}
