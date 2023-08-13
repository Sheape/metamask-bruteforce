use crate::address::Wallets;
use reqwest::{IntoUrl, Client};
use strum_macros::EnumIter;
use serde::Deserialize;

use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use dotenv::dotenv;
use std::env;

pub(crate) static BSCSCAN_PREFIX_URL: &str = "https://api.bscscan.com";
pub(crate) static ETHERSCAN_PREFIX_URL: &str = "https://api.etherscan.io";
pub(crate) static POLYGONSCAN_PREFIX_URL: &str = "https://api.polygonscan.com";
pub(crate) static ARBISCAN_PREFIX_URL: &str = "https://api.arbiscan.io";
pub(crate) static FTMSCAN_PREFIX_URL: &str = "https://api.ftmscan.com";
pub(crate) static OPTIMISTIC_PREFIX_URL: &str = "https://api-optimistic.etherscan.io";
pub(crate) static CRONOSCAN_PREFIX_URL: &str = "https://api.cronoscan.com";
pub(crate) static BTTCSCAN_PREFIX_URL: &str = "https://api.bttcscan.com";
pub(crate) static MOONBEAM_PREFIX_URL: &str = "https://api-moonbeam.moonscan.io";
pub(crate) static MOONRIVER_PREFIX_URL: &str = "https://api-moonriver.moonscan.io";
pub(crate) static SNOWTRACE_PREFIX_URL: &str = "https://api.snowtrace.io";
pub(crate) static CELOSCAN_PREFIX_URL: &str = "https://api.celoscan.io";
pub(crate) static BOBASCAN_PREFIX_URL: &str = "https://api.bobascan.com";
pub(crate) static GNOSISSCAN_PREFIX_URL: &str = "https://api.gnosisscan.io";

/// Chain type
// TODO: Temporary Debug derive
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum ChainType {
    /// Binance Smart Chain
    BSC,

    /// Ethereum
    Ethereum,

    /// Polygon
    Polygon,

    /// Arbitrum
    Arbitrum,

    /// Fantom
    Fantom,

    /// Optimism
    Optimism,

    /// Cronos
    Cronos,

    /// Moonbeam
    Moonbeam,

    /// Moonriver
    Moonriver,

    /// BitTorrent
    BitTorrent,

    /// Avalanche
    Avalanche,

    /// Celo
    Celo,

    /// Boba
    Boba,

    /// Gnosis
    Gnosis,
}


pub fn get_prefix_url(chain: ChainType) -> &'static str {
    match chain {
        ChainType::BSC => BSCSCAN_PREFIX_URL,
        ChainType::Ethereum => ETHERSCAN_PREFIX_URL,
        ChainType::Polygon => POLYGONSCAN_PREFIX_URL,
        ChainType::Arbitrum => ARBISCAN_PREFIX_URL,
        ChainType::Fantom => FTMSCAN_PREFIX_URL,
        ChainType::Optimism => OPTIMISTIC_PREFIX_URL,
        ChainType::Cronos => CRONOSCAN_PREFIX_URL,
        ChainType::Moonbeam => MOONBEAM_PREFIX_URL,
        ChainType::Moonriver => MOONRIVER_PREFIX_URL,
        ChainType::BitTorrent => BTTCSCAN_PREFIX_URL,
        ChainType::Avalanche => SNOWTRACE_PREFIX_URL,
        ChainType::Celo => CELOSCAN_PREFIX_URL,
        ChainType::Boba => BOBASCAN_PREFIX_URL,
        ChainType::Gnosis => GNOSISSCAN_PREFIX_URL
    }
}

pub fn get_api_key(chain: ChainType) -> String {
    match chain {
        ChainType::BSC => env::var("BSC_SCAN").unwrap(),
        ChainType::Ethereum => env::var("ETHER_SCAN").unwrap(),
        ChainType::Polygon => env::var("POLYGON_SCAN").unwrap(),
        ChainType::Arbitrum => env::var("ARB_SCAN").unwrap(),
        ChainType::Fantom => env::var("FTM_SCAN").unwrap(),
        ChainType::Optimism => env::var("OPT_SCAN").unwrap(),
        ChainType::Cronos => env::var("CRO_SCAN").unwrap(),
        ChainType::Moonbeam => env::var("GLMR_SCAN").unwrap(),
        ChainType::Moonriver => env::var("MOVR_SCAN").unwrap(),
        ChainType::BitTorrent => env::var("BTT_SCAN").unwrap(),
        ChainType::Avalanche => env::var("AVAX_SCAN").unwrap(),
        ChainType::Celo => env::var("CELO_SCAN").unwrap(),
        ChainType::Boba => env::var("BOBA_SCAN").unwrap(),
        ChainType::Gnosis => env::var("XDAI_SCAN").unwrap()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Account {
    pub account: String,
    pub balance: String
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    status: String,
    message: String,
    result: Vec<Account>
}

#[derive(Debug)]
pub struct MergedAddress {
    pub address: String,
    pub mnemonic: String,
    pub balance: String,
}

// impl Account {
//     pub fn
// }
const OUTPUT_FILE: &str = "found.txt";

impl MergedAddress {
    pub async fn check_if_not_empty(&self, chain: ChainType) -> Result<(), Box<dyn std::error::Error>> {
        if self.balance != "0" {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(OUTPUT_FILE)
                .await?;
            let data = format!("Address: {address}\nMnemonic: {mnemonic}\nBalance: {balance}\nChain: {chain:?}\n\n",
                                address = self.address,
                                mnemonic = self.mnemonic,
                                balance = self.balance,
                                chain = chain,
                                );

            file.write_all(data.as_bytes()).await?;
        }
        Ok(())
    } 
}

pub async fn get_multiple_address(addresses: Wallets, chain: ChainType, client: &Client)
        -> Result<Vec<MergedAddress>, reqwest::Error> {
    dotenv().ok();
    let addr = addresses.clone().addresses_to_str();
    let prefix_url = get_prefix_url(chain);
    let api_key = get_api_key(chain);
    let url = format!(
        "{prefix_url}/api?module=account&action=balancemulti&address={addresses_str}&tag=latest&apikey={api_key}",
        prefix_url = prefix_url,
        addresses_str = addr,
        api_key = api_key
    );

    let response = client.get(url).send().await.unwrap();

    let body = response.json::<ResponseData>().await.unwrap();

    // println!("{:?}", body.result);
    let merged_vector: Vec<MergedAddress> = addresses
                        .wallets.iter()
                        .zip(body.result.iter())
                        .map(|(wallets_struct, response_struct)| MergedAddress {
                            // TODO: Optimize later to avoid using clone
                            address: wallets_struct.clone().address,
                            mnemonic: wallets_struct.clone().mnemonic,
                            balance: response_struct.clone().balance,
                        })
                        .collect();
    for merged_address in merged_vector.iter() {
        merged_address.check_if_not_empty(chain).await.unwrap();
    }
    Ok(merged_vector)
}
