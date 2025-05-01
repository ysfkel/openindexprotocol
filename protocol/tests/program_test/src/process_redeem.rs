use openindex_sdk::openindex::transaction::redeem_transaction;
use solana_sdk::pubkey::Pubkey;

use crate::{ProcessRedeemResult, Setup};

pub async fn process_redeem(
    amount: u64,
    index_id: u64,
    controller_id: u64,
    token_account: Pubkey,
    mints: Vec<Pubkey>,
    token_accounts: Vec<Pubkey>,
    _setup: &Setup,
) -> ProcessRedeemResult {
    let payer = &_setup.payer;
    let program_id = _setup.program_id;
    let recent_blockhashes = _setup.recent_blockhashes;

    let redeem_tx = redeem_transaction(
        amount,
        payer,
        program_id,
        index_id,
        controller_id,
        token_account,
        recent_blockhashes,
        mints,
        token_accounts,
    );

    let result = _setup.banks_client.process_transaction(redeem_tx).await;
    ProcessRedeemResult { result }
}
