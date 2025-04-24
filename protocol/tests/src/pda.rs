// use open_index_lib::seeds::{
//     COMPONENT_SEED, COMPONENT_VAULT_SEED, CONTROLLER_GLOBAL_CONFIG_SEED, CONTROLLER_SEED,
//     INDEX_MINTS_DATA_SEED, INDEX_MINT_AUTHORITY_SEED, INDEX_MINT_SEED, INDEX_SEED, MODULE_SEED,
//     PROTOCOL_SEED,
// };
// use solana_sdk::pubkey::Pubkey;

// pub fn find_protocol_address(program_id: &Pubkey) -> (Pubkey, u8) {
//     let (pda, bump) = Pubkey::find_program_address(&[PROTOCOL_SEED], program_id);
//     (pda, bump)
// }

// pub fn find_controller_address(program_id: &Pubkey, controller_id: u64) -> (Pubkey, u8) {
//     let (pda, bump) = Pubkey::find_program_address(
//         &[CONTROLLER_SEED, &controller_id.to_le_bytes()],
//         &program_id,
//     );
//     (pda, bump)
// }

// pub fn find_index_address(
//     program_id: &Pubkey,
//     controller_key: &Pubkey,
//     index_id: u64,
// ) -> (Pubkey, u8) {
//     let (pda, bump) = Pubkey::find_program_address(
//         &[INDEX_SEED, controller_key.as_ref(), &index_id.to_le_bytes()],
//         program_id,
//     );
//     (pda, bump)
// }

// pub fn find_controller_global_config_address(program_id: &Pubkey) -> (Pubkey, u8) {
//     let (pda, bump) = Pubkey::find_program_address(&[CONTROLLER_GLOBAL_CONFIG_SEED], program_id);
//     (pda, bump)
// }

// pub fn find_index_mint_address(
//     program_id: &Pubkey,
//     controller_account: &Pubkey,
//     index_id: u64,
// ) -> (Pubkey, u8) {
//     let (pda, bump) = Pubkey::find_program_address(
//         &[
//             INDEX_MINT_SEED,
//             controller_account.as_ref(),
//             &index_id.to_le_bytes(),
//         ],
//         program_id,
//     );
//     (pda, bump)
// }

// pub fn find_index_mints_data_address(
//     program_id: &Pubkey,
//     controller_account: &Pubkey,
//     index_id: u64,
// ) -> (Pubkey, u8) {
//     let (pda, bump) = Pubkey::find_program_address(
//         &[
//             INDEX_MINTS_DATA_SEED,
//             controller_account.as_ref(),
//             &index_id.to_le_bytes(),
//         ],
//         program_id,
//     );
//     (pda, bump)
// }

// pub fn find_component_address(
//     program_id: &Pubkey,
//     index_key: &Pubkey,
//     mint_key: &Pubkey,
// ) -> (Pubkey, u8) {
//     let (pda, bump) = Pubkey::find_program_address(
//         &[COMPONENT_SEED, index_key.as_ref(), mint_key.as_ref()],
//         program_id,
//     );
//     (pda, bump)
// }

// pub fn find_component_vault_address(
//     program_id: &Pubkey,
//     index_key: &Pubkey,
//     mint_key: &Pubkey,
// ) -> (Pubkey, u8) {
//     let (pda, bump) = Pubkey::find_program_address(
//         &[COMPONENT_VAULT_SEED, index_key.as_ref(), mint_key.as_ref()],
//         program_id,
//     );
//     (pda, bump)
// }

// pub fn find_module_signer_address(program_id: &Pubkey) -> (Pubkey, u8) {
//     let (pda, bump) = Pubkey::find_program_address(&[program_id.as_ref()], program_id);
//     (pda, bump)
// }

// pub fn find_registered_module_address(
//     program_id: &Pubkey,
//     module_signer_account: &Pubkey,
// ) -> (Pubkey, u8) {
//     let (pda, bump) =
//         Pubkey::find_program_address(&[&MODULE_SEED, &module_signer_account.as_ref()], program_id);
//     (pda, bump)
// }

// pub fn find_index_mint_authority_address(
//     program_id: &Pubkey,
//     controller_account: &Pubkey,
//     index_id: u64,
// ) -> (Pubkey, u8) {
//     let (pda, nump) = Pubkey::find_program_address(
//         &[
//             INDEX_MINT_AUTHORITY_SEED,
//             controller_account.as_ref(),
//             &index_id.to_le_bytes(),
//         ],
//         program_id,
//     );
//     (pda, nump)
// }
