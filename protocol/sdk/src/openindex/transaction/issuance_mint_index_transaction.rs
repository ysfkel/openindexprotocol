// use crate::pda::{
//     find_controller_address, find_index_address, find_index_mint_address,
//     find_index_mint_authority_address, find_index_mints_data_address, find_module_signer_address,
//     find_registered_module_address,
// };
// use solana_program::example_mocks::solana_keypair::Keypair;
// use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

// pub fn issuance_mint_index_transaction(
//     payer: &Keypair,
//     open_inddex_program_id: Pubkey,
//     issuance_program_id: Pubkey,
//     index_id: u64,
//     amount: u64,
//     controller_id: u64,
//     token_account: &Pubkey,
//     mints: Vec<Pubkey>,
 
// ) -> Instruction {

//     let issuance_program_id = &issuance_program_id;
//     let module_signer_pda = find_module_signer_address(issuance_program_id).0;
//     let registered_module_account =
//         find_registered_module_address(issuance_program_id, &module_signer_pda).0;
//     let controller_pda = find_controller_address(&open_inddex_program_id, controller_id).0;
//     let mint_pda = find_index_mint_address(&open_inddex_program_id, &controller_pda, index_id).0;
//     let mint_authourity =
//         find_index_mint_authority_address(&open_inddex_program_id, &controller_pda, index_id).0;
//     let index_pda = find_index_address(&open_inddex_program_id, &controller_pda, index_id).0;
//     let index_mints_data_pda =
//         find_index_mints_data_address(&open_inddex_program_id, &controller_pda, index_id).0;

//       //  Instruction {}
// }

// // let caller_account = next_account_info(accounts_iter)?;
// // let module_account = next_account_info(accounts_iter)?;
// // let registered_module_account = next_account_info(accounts_iter)?;
// // let controller_account = next_account_info(accounts_iter)?;
// // let mint_account = next_account_info(accounts_iter)?;
// // let mint_authority_account = next_account_info(accounts_iter)?;
// // let index_account = next_account_info(accounts_iter)?;
// // let index_mints_account = next_account_info(accounts_iter)?;

// // let open_index_account = next_account_info(accounts_iter)?;
// // let token_account = next_account_info(accounts_iter)?;
// // let token_program_account = next_account_info(accounts_iter)?;
