use super::AccountType;

pub struct IndexModule {
    pub account_type: AccountType,
    pub is_active: bool,
    pub initialized: bool,
    pub bump: u8,
}
