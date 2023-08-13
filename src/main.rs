use futures::StreamExt;
use metamask_bruteforce::{
    EthWallet,
    Wallets,
    ChainType,
    FinalWallet,
    get_multiple_address,
    setup_terminal,
    run,
    restore_terminal,
    merge_balances
};
use reqwest::Client;
use strum::IntoEnumIterator;

use std::time::Instant;
use std::error;
use futures::stream::FuturesOrdered;
use tokio::task::JoinSet;

use std::{
    io::{self, Stdout},
    time::Duration,
    error::Error
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
// use ratatui::{Terminal, prelude::*, widgets::Paragraph};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    // Initializing HTTP Client
    let client= Client::builder().build().unwrap();

    let mut terminal = setup_terminal()?;
    run(&mut terminal, client).await?;
    restore_terminal(&mut terminal)?;
    Ok(())
}
