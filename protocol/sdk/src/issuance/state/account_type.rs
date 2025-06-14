//! Account types

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
pub enum AccountType {
    /// Default uninitialized account state
    #[default]
    IssuanceConfig,
  
}
