mod address;
mod api;
mod ui;
mod generator;
// use std::fmt;

pub use address::{ EthWallet, FinalWallet, Wallets, merge_balances};
pub use api::{ChainType, get_multiple_address};
pub use ui::{setup_terminal, run, restore_terminal};
pub use generator::generate_batches;
