//! Account types 

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
pub enum AccountType {
    /// Default uninitialized account state
    #[default]
    Uninitialized,
    /// The protocol account - stores protocol config
    Protocol,
    /// The controller account - stores the controller data
    Controller,
     /// The controller global config account - stores global config for the controllers
    ControllerGlobalConfig, 
    /// The index account
    Index,
    /// The component account - index component data
    Component,
    /// The index mints account - stores mint addresses that are part of the index
    IndexMints, 
    /// Module account -  Determines if an external program is a registered module
    Module, 
}