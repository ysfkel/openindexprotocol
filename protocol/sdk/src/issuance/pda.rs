use solana_program::pubkey::Pubkey;

use crate::issuance::seeds::ISSUANCE_CONFIG_SEED;

pub fn find_issuance_signer_address(program_id: &Pubkey) -> (Pubkey, u8) {
    let (pda, bump) = Pubkey::find_program_address(&[program_id.as_ref()], program_id);
    (pda, bump)
}
pub fn find_issuance_config_address(program_id: &Pubkey) -> (Pubkey, u8) {
    let (pda, bump) = Pubkey::find_program_address(&[ISSUANCE_CONFIG_SEED], program_id);
    (pda, bump)
}
