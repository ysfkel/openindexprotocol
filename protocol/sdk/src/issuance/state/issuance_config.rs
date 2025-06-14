use solana_program::pubkey::Pubkey;

pub struct IssuanceConfig {
  pub allowed_hooks:  Option<Vec<Pubkey>>,
  pub bump: u8,
}
