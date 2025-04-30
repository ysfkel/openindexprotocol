use borsh::BorshDeserialize;
use openindex::state::Controller;
use openindex_sdk::openindex::{
    pda::find_controller_address, transaction::create_index_transaction,
};
use solana_sdk::pubkey::Pubkey;

use crate::{ProcessCreateIndexResult, Setup};

pub async fn process_create_index(
    controller_id: u64,
    manager: Pubkey,
    _setup: &Setup,
) -> ProcessCreateIndexResult {
    let program_id = _setup.program_id;
    let controller_pda = find_controller_address(&program_id, controller_id).0;

    let controller_account = _setup
        .banks_client
        .get_account(controller_pda)
        .await
        .unwrap()
        .unwrap();

    let controller: Controller = Controller::try_from_slice(&controller_account.data).unwrap();

    let create_index_tx = create_index_transaction(
        &_setup.payer,
        program_id,
        controller.next_index_id,
        controller_id,
        manager,
        _setup.recent_blockhashes,
    );

    let result = _setup
        .banks_client
        .process_transaction(create_index_tx)
        .await;

    ProcessCreateIndexResult {
        index_id: controller.next_index_id,
        controller_pda,
        result,
    }
}
