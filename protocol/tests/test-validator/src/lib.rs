#![cfg(all(test,feature = "test-validator"))]

mod test_add_index_components;
mod read_keys;
mod setup;

pub use read_keys::*;
pub use setup::*; 