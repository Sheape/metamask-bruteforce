use futures::stream::FuturesOrdered;
use futures::StreamExt;

use strum::IntoEnumIterator;
use crate::{FinalWallet, Wallets, EthWallet, ChainType, merge_balances, get_multiple_address};
use std::error;

pub async fn generate_batches() -> Result<Vec<FinalWallet>, Box<dyn error::Error + Send + Sync>> {
    let wallets = EthWallet::batch().await.unwrap();
    // let wallets = Wallets::new(vec![EthWallet {
    //     address: String::from("0x3f349bBaFEc1551819B8be1EfEA2fC46cA749aA1"),
    //     mnemonic: String::from("immune two lemon gloom place resource walk stay half trend mercy bus")
    // },
    // EthWallet {
    //     address: String::from("0xc7456c0b41990a08ab0b9b702ed565bddcbf7216"),
    //     mnemonic: String::from("rally shop feature lava pause jealous body edge swear exercise fuel rug")
    // },
    // EthWallet {
    //     address: String::from("0x7a56642741a3c94c2265271d5ef5aeffa4a23aeb"),
    //     mnemonic: String::from("disease margin power detect price link erode universe engine maid squirrel bargain")
    // },
    // EthWallet {
    //     address: String::from("0x2a58d9cfcbebad4297c8eddbe06973ce7b3c83aa"),
    //     mnemonic: String::from("town burger claw believe swift year credit slush purity sample panda subway")
    // },
    // EthWallet {
    //     address: String::from("0x5eb8eaffb597045725f9d5eb02c469e8459fe3f0"),
    //     mnemonic: String::from("noodle violin chunk nominee little path print mirror couch access stem boring")
    // },
    // EthWallet {
    //     address: String::from("0x737ae7efe4946e32633fc96519e973c4b6555ef0"),
    //     mnemonic: String::from("gossip involve burger pretty scene hospital assault leisure arctic nut style credit")
    // },
    // EthWallet {
    //     address: String::from("0xa10c1e1f9865d033da7d2bd1063e6765ae51fb20"),
    //     mnemonic: String::from("rather consider drill enemy obvious pipe couch purity call hurdle behind piano")
    // },
    // EthWallet {
    //     address: String::from("0x192f4d046cf88cd8e92e285bb2e1085e82217fdb"),
    //     mnemonic: String::from("predict equal jump combine artefact hold weird pupil attack frog circle elite")
    // },
    // EthWallet {
    //     address: String::from("0x7c0faa259a6cddc5ce6437a7cfc7bf40989426f8"),
    //     mnemonic: String::from("found that become truth version please funny start foam lounge morning absurd")
    // },
    // EthWallet {
    //     address: String::from("0x2485b8dfc0e6c7150a26646acf863c7c2e38b05b"),
    //     mnemonic: String::from("mistake inquiry blur until powder bundle century flash dizzy baby cry swear")
    // },
    // EthWallet {
    //     address: String::from("0x055df719fd06ca19f66a1f274263f595cca9484e"),
    //     mnemonic: String::from("relief interest limit fitness aunt accuse river lawsuit always unusual goose manage")
    // },
    // EthWallet {
    //     address: String::from("0xc7ab81a7da8ad58c428f77f1595c415fcd25d625"),
    //     mnemonic: String::from("heart deal congress solid aisle tomato sign bitter enrich pizza episode nuclear")
    // },
    // EthWallet {
    //     address: String::from("0x3a52c4258601ed352fa4d4fcb172027565e5301d"),
    //     mnemonic: String::from("sail tornado post portion satoshi furnace unfair also start learn antenna neglect")
    // },
    // EthWallet {
    //     address: String::from("0x6e44f4537ef8e91be5db7e79b6f3114a3f332c6b"),
    //     mnemonic: String::from("fat valid submit direct window arrange embark symbol sing series blue fiber")
    // },
    // EthWallet {
    //     address: String::from("0xe9093ba5706c59472709f7c138013af53760f674"),
    //     mnemonic: String::from("vacuum domain rather total ostrich coin motion rhythm essence vocal chase bean")
    // },
    // EthWallet {
    //     address: String::from("0x7e21057fa0a794a539c3b122cc166d799e81448b"),
    //     mnemonic: String::from("dinner olympic process army rally kitchen size uniform limb reopen sweet obvious")
    // },
    // EthWallet {
    //     address: String::from("0x1bb913ed3b67ba944a7fbc2b6df305a68a020933"),
    //     mnemonic: String::from("payment rate engage embrace someone copy renew copy same banner lobster toss")
    // },
    // EthWallet {
    //     address: String::from("0x4b28f419697b2472507650aa70aa538e85b843fb"),
    //     mnemonic: String::from("siren certain stamp shoulder tunnel romance burden success extend share pupil replace")
    // },
    // EthWallet {
    //     address: String::from("0xeb39d076b2d421158f3c339cbe17065187991459"),
    //     mnemonic: String::from("shove spot brown adult moment annual skull sure extend worry custom profit")
    // },
    // EthWallet {
    //     address: String::from("0x67b9b0f3a7e9793ebaee3a51fbd343a2775391a8"),
    //     mnemonic: String::from("owner auction exit apple fold list share twelve another layer chase menu")
    // }
    // ]);

    let mut tasks = FuturesOrdered::new();
    for chain in ChainType::iter() {
        // TODO: Optimize later using Arc for safe-thread referencing.
        // println!("{:?}", chain);
        tasks.push_back(get_multiple_address(wallets.clone(), chain));
    }

    let mut final_results = vec![];
    while let Some(res) = tasks.next().await {
        final_results.push(res.unwrap());
    }

    Ok(merge_balances(final_results))
    // println!("{:?}", actual_final_results);
}
