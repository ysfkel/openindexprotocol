// #![cfg(all(test, feature = "test-validator"))]

mod process_mint;
mod read_keys;
mod setup;
mod test_add_index_components;
// mod add_token_metadata;

pub use process_mint::*;
pub use read_keys::*;
pub use setup::*;
