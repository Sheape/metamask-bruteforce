use bip39::{ Mnemonic, Language };
use rand::RngCore;
use ethers_signers::{ MnemonicBuilder, coins_bip39::English, Signer };
use tokio::task;

use crate::api::MergedAddress;

#[derive(Debug, Clone)]
pub struct EthWallet {
    pub address: String,
    pub mnemonic: String,
}

#[derive(Clone)]
pub struct Wallets {
    pub wallets: Vec<EthWallet>
}

#[derive(Debug, Clone)]
pub struct ChainBalance {
    pub bsc: String,
    pub ethereum: String,
    pub polygon: String,
    pub polygon_zkevm: String,
    pub arbitrum: String,
    pub fantom: String,
    pub optimism: String,
    pub cronos: String,
    pub moonbeam: String,
    pub moonriver: String,
    pub bittorrent: String,
    pub avalanche: String,
    pub celo: String,
    pub boba: String,
    pub gnosis: String,
}

#[derive(Debug, Clone)]
pub struct FinalWallet {
    pub address: String,
    pub mnemonic: String,
    pub balances: ChainBalance
}

impl EthWallet {
    pub async fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut entropy: [u8; 16] = [0; 16];
        rng.fill_bytes(&mut entropy);

        let mnemonic = Mnemonic::from_entropy(&entropy, Language::English).unwrap();

        // PERF: Definitely change this to wagyu for a lighter library
        let address = MnemonicBuilder::<English>::default()
            .phrase(mnemonic.phrase())
            .index(0u32)
            .unwrap()
            .build()
            .unwrap();

        EthWallet { address: format!("{:?}", address.address()), mnemonic: mnemonic.into_phrase()}
    }

    // Uses FuturesUnordered
    pub async fn batch() -> Result<Wallets, task::JoinError> {
        let mut batch = task::JoinSet::new();
        let mut results = Vec::with_capacity(20);
        for _ in 0..20 {
            batch.spawn(EthWallet::new());
        }
        while let Some(res) = batch.join_next().await {
            results.push(res.unwrap());
        };
        Ok(Wallets { wallets: results })
    }
}

impl Wallets {
    pub fn new(wallets: Vec<EthWallet>) -> Self {
        Wallets { wallets }
    }

    pub fn addresses_to_str(self) -> String {
        // addresses.iter().map(|wallet| wallet.address.clone()).collect();
        let addresses: Vec<String> = self.wallets.iter().map(|wallet| wallet.address.clone()).collect();
        addresses.join(", ")
    }
}

// TODO: Optimize this later to avoid cloning
pub fn merge_balances(results: Vec<Vec<MergedAddress>>) -> Vec<FinalWallet> {
    let mut final_results = Vec::with_capacity(20);
    for set_of_response in results.iter() {
        for (j, merged_address) in set_of_response.iter().enumerate() {
            let final_wallet = FinalWallet {
                address: merged_address.address.clone(),
                mnemonic: merged_address.mnemonic.clone(),
                balances: ChainBalance {
                    bsc:           results[0][j].balance.clone(),
                    ethereum:      results[1][j].balance.clone(),
                    polygon:       results[2][j].balance.clone(),
                    polygon_zkevm: results[3][j].balance.clone(),
                    arbitrum:      results[4][j].balance.clone(),
                    fantom:        results[5][j].balance.clone(),
                    optimism:      results[6][j].balance.clone(),
                    cronos:        results[7][j].balance.clone(),
                    moonbeam:      results[8][j].balance.clone(),
                    moonriver:     results[9][j].balance.clone(),
                    bittorrent:    results[10][j].balance.clone(),
                    avalanche:     results[11][j].balance.clone(),
                    celo:          results[12][j].balance.clone(),
                    boba:          results[13][j].balance.clone(),
                    gnosis:        results[14][j].balance.clone()
                }
            };
            if final_results.len() < 20 {
                final_results.push(final_wallet);
            }
        }
    }
    final_results
}
