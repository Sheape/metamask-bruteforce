use metamask_bruteforce::{
    setup_terminal,
    run,
    restore_terminal,
};
use reqwest::Client;
use std::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    // Initializing HTTP Client
    let client= Client::builder().build().unwrap();

    let mut terminal = setup_terminal()?;
    run(&mut terminal, client).await?;
    restore_terminal(&mut terminal)?;
    Ok(())
}
