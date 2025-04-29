// use borsh::{BorshDeserialize, BorshSerialize};
// use solana_program::{instruction::Instruction, pubkey::Pubkey};

// #[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
// pub enum IssuanceInstruction {
//     Mint { index_id: u64, amount: u64 },
//     Redeem,
// }

// #[allow(clippy::too_many_arguments)]
// pub fn issue_instruction(
//     index_id: u64,
//     amount: u64,
//     token_account: Pubkey,
//     program_id: Pubkey,
//     caller_account: Pubkey,
//     module_signer_account: Pubkey,
//     registered_module_account: Pubkey,
//     controller_account: Pubkey,
//     mint_account: Pubkey,
//     mint_authority_account: Pubkey,
//     index_account: Pubkey,
//     index_mints_data_account: Pubkey,
//     open_index_program_id: Pubkey,
// ) -> Instruction {
//     let mut accounts = vec![
//         AccountMeta::new(caller_account, true),
//         AccountMeta::new(module_signer_account, false),
//         AccountMeta::new(registered_module_account, false),
//         AccountMeta::new(controller_account, false),
//         AccountMeta::new(mint_account, false),
//         AccountMeta::new(mint_authority_account, false),
//         AccountMeta::new(index_account, false),
//         AccountMeta::new(index_mints_data_account, false),
//         AccountMeta::new(open_index_program_id, false),
//         AccountMeta::new(token_account, false),
//         AccountMeta::new_readonly(system_program::ID, false),
//         AccountMeta::new_readonly(spl_associated_token_account::ID, false),
//         AccountMeta::new_readonly(spl_token::ID, false),
//     ];

//     let instruction = IssuanceInstruction::Mint { index_id, amount };
//     let data = borsh::to_vec(&instruction).unwrap();
//     Instruction {
//         program_id,
//         accounts,
//         data,
//     }
// }

// pub fn module_issuance_mint_index_transaction(
//     index_id: u64,
//     amount: u64,
//     controller_id: u64,
//     token_account: &Pubkey,
//     mints: Vec<Pubkey>,
//     _setup: &Setup,
// ) -> ModuleMintIndexTransaction {
//        let caller = &_setup.payer;
//        let program_id = &_setup.program_id;
//        let issuance_program_id = &_setup.issuance_program_id;
//        let module_signer_pda = find_module_signer_address(issuance_program_id).0;
//        let registered_module_account = find_registered_module_address(issuance_program_id, &module_signer_pda).0;
//        let controller_pda = find_controller_address(program_id, controller_id).0;
//        let mint_pda = find_index_mint_address(program_id, &controller_pda, index_id).0;
//        let mint_authourity =  find_index_mint_authority_address(program_id,&controller_pda, index_id).0;
//        let index_pda = find_index_address(program_id, &controller_pda, index_id).0;
//        let index_mints_data_pda = find_index_mints_data_address(program_id, &controller_pda, index_id).0;

//     ModuleMintIndexTransaction {}
// }
