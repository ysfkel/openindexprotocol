use open_index_lib::seeds::{CONTROLLER_SEED, PROTOCOL_SEED};
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
