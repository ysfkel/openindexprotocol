use solana_sdk::pubkey::Pubkey;

use crate::{
    find_controller_address, find_index_address, find_index_mint_authority_address,
    find_index_mints_data_address, find_module_signer_address, find_registered_module_address,
    get_index_mint_pda, Setup,
};

pub struct ModuleMintIndexTransaction {}
pub fn module_issuance_mint_index_transaction(
    index_id: u64,
    amount: u64,
    controller_id: u64,
    token_account: &Pubkey,
    mints: Vec<Pubkey>,
    _setup: &Setup,
) -> ModuleMintIndexTransaction {
    //    let caller = &_setup.payer;
    //    let program_id = &_setup.program_id;
    //    let issuance_program_id = &_setup.issuance_program_id;
    //    let module_signer_pda = find_module_signer_address(issuance_program_id).0;
    //    let registered_module_account = find_registered_module_address(issuance_program_id, &module_signer_pda).0;
    //    let controller_pda = find_controller_address(program_id, controller_id).0;
    //    let mint_pda = get_index_mint_pda(program_id, &controller_pda, index_id).0;
    //    let mint_authourity =  find_index_mint_authority_address(program_id,&controller_pda, index_id).0;
    //    let index_pda = find_index_address(program_id, &controller_pda, index_id).0;
    //    let index_mints_data_pda = find_index_mints_data_address(program_id, &controller_pda, index_id).0;

    ModuleMintIndexTransaction {}
}

// let caller_account = next_account_info(accounts_iter)?;
// let module_account = next_account_info(accounts_iter)?;
// let registered_module_account = next_account_info(accounts_iter)?;
// let controller_account = next_account_info(accounts_iter)?;
// let mint_account = next_account_info(accounts_iter)?;
// let mint_authority_account = next_account_info(accounts_iter)?;
// let index_account = next_account_info(accounts_iter)?;
// let index_mints_account = next_account_info(accounts_iter)?;

// let open_index_account = next_account_info(accounts_iter)?;
// let token_account = next_account_info(accounts_iter)?;
// let token_program_account = next_account_info(accounts_iter)?;
