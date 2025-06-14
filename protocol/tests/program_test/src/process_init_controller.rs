use borsh::BorshDeserialize;
use openindex_sdk::openindex::{
    pda::{find_controller_address, find_protocol_address},
    state::Protocol,
    transaction::init_controller_transaction,
};

use crate::{ProcessInitControllerResult, Setup};

pub async fn process_init_controller(_setup: &Setup) -> ProcessInitControllerResult {
    let program_id = _setup.program_id;
    let protocol_pda = find_protocol_address(&program_id).0;
    let protocol_account = _setup
        .banks_client
        .get_account(protocol_pda)
        .await
        .unwrap()
        .unwrap();

    let protocol: Protocol = Protocol::try_from_slice(&protocol_account.data).unwrap();
    let controller_id = protocol.get_next_controller_id();
    let controller_pda = find_controller_address(&program_id, controller_id).0;
    let int_controller_tx = init_controller_transaction(
        &_setup.payer,
        program_id,
        controller_id,
        _setup.recent_blockhashes,
    );
    let result = _setup
        .banks_client
        .process_transaction(int_controller_tx.clone())
        .await;

    ProcessInitControllerResult {
        controller_id,
        controller_pda,
        result,
    }
}
