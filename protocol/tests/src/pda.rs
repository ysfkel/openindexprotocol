use open_index_lib::seeds::{
    COMPONENT_SEED, COMPONENT_VAULT_SEED, CONTROLLER_GLOBAL_CONFIG_SEED, CONTROLLER_SEED, INDEX_MINTS_DATA_SEED, INDEX_MINT_SEED, INDEX_SEED, PROTOCOL_SEED
};
use solana_sdk::pubkey::Pubkey;

pub fn get_protocol_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    let (pda, bump) = Pubkey::find_program_address(&[PROTOCOL_SEED], program_id);
    (pda, bump)
}

pub fn get_controller_pda(program_id: &Pubkey, controller_id: u64) -> (Pubkey, u8) {
    let (pda, bump) = Pubkey::find_program_address(
        &[CONTROLLER_SEED, &controller_id.to_le_bytes()],
        &program_id,
    );
    (pda, bump)
}

pub fn get_index_pda(program_id: &Pubkey, controller_key: &Pubkey, index_id: u64) -> (Pubkey, u8) {
    let (pda, bump) = Pubkey::find_program_address(
        &[INDEX_SEED, controller_key.as_ref(), &index_id.to_le_bytes()],
        program_id,
    );
    (pda, bump)
}

pub fn get_controller_global_config_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    let (pda, bump) = Pubkey::find_program_address(&[CONTROLLER_GLOBAL_CONFIG_SEED], program_id);
    (pda, bump)
}

pub fn get_index_mint_pda(
    program_id: &Pubkey,
    controller_account: &Pubkey,
    index_id: u64,
) -> (Pubkey, u8) {
    let (pda, bump) = Pubkey::find_program_address(
        &[
            INDEX_MINT_SEED,
            controller_account.as_ref(),
            &index_id.to_le_bytes(),
        ],
        program_id,
    );
    (pda, bump)
}

pub fn find_index_mints_data_address(
    program_id: &Pubkey,
    controller_account: &Pubkey,
    index_id: u64,
) -> (Pubkey, u8) {
    let (pda, bump) = Pubkey::find_program_address(
        &[
            INDEX_MINTS_DATA_SEED,
            controller_account.as_ref(),
            &index_id.to_le_bytes(),
        ],
        program_id,
    );
    (pda, bump)
}

pub fn find_component_address(program_id: &Pubkey, index_key: &Pubkey, mint_key: &Pubkey) -> (Pubkey, u8) {

    let (pda, bump) = Pubkey::find_program_address(
        &[
            COMPONENT_SEED,
            index_key.as_ref(),
            mint_key.as_ref(),
        ],
        program_id,
    );
    (pda, bump)
}

pub fn find_component_vault_address(program_id: &Pubkey, index_key: &Pubkey, mint_key: &Pubkey) -> (Pubkey, u8) {

    let (pda, bump) = Pubkey::find_program_address(
        &[
            COMPONENT_VAULT_SEED,
            index_key.as_ref(),
            mint_key.as_ref(),
        ],
        program_id,
    );
    (pda, bump)
}