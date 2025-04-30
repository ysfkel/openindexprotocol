use openindex_sdk::openindex::transaction::init_protocol_transaction;

use crate::{BanksClientResult, ProcessInitProtocolResult, Setup};

pub async fn process_init_protocol(_setup: &Setup) -> ProcessInitProtocolResult {
    let transaction =
        init_protocol_transaction(&_setup.payer, _setup.program_id, _setup.recent_blockhashes);

    let result = _setup
        .banks_client
        .process_transaction(transaction.clone())
        .await;

    ProcessInitProtocolResult { result }
}
