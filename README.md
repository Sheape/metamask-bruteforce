

# Metamask Bruteforce

This is my first project written in rust, so please if you have any issues feel
free to open them or if you have suggestions on how I can improve my code. It's
not actually bruteforcing metamask but rather just using Blockscan's API.

![img](https://user-images.githubusercontent.com/86521166/260396429-eb877d79-82c3-4301-900d-cd99401fcde8.png)


## Why?

People tend to forget their mnemonics or passwords, or even send someone
ethereum but in the wrong network/address. There are still some lost wallets
which have balance in them and nobody can access them. Metamask isn't really the
target of this but it just happens to be very popular with a lot of ethereum
networks.

It is not illegal to try to find these wallets. Just like a treasure chest or
lost pile of gold, the owner of them either died or just forgot about it and if
you're lucky to find one, then you can claim it for yours.

Dont steal active wallets, Im not responsible for what you would do with this.
It's not my responsibility if you do something wrong. It takes so much time to
find one lost wallet so goodluck!


## Features

-   **Low memory usage** - Only using about <ins>20MB of RAM</ins>, so you can run this without
    affecting your daily workflow.
-   **Minimum CPU usage** - Only one of spike of CPU every second. Unlike other options
    where hashing will inevitably impact your CPU.
-   **Realiable** - It uses an [online API](https://blockscan.com) to check the balance of the address instead
    of doing it offline where the user is required to install a very big database.
-   **Blazingly fast** - It maximizes the performance while scanning all of the
     generated address and still maintaining a low memory usage using <ins>asynchronous
    code</ins>.
-   **Binary-only**: Unlike other scripts where it is interpreted at runtime, since
    rust compiles a program to a binary, there is no need for you to install an
    interpreter or compiler.


## How it works?

This works by efficiently generating 12 words mnemonic and hashing it to an
ethereum address. 100 pairs are generated in a matter of `50ms` which is
extremely fast and only the network and API calls take much of the time.

It uses [Blockscan](https://blockscan.com/) APIs and its incredible ecosystem for each of the network as
they all have the same framework of API. Each API call process <ins>20 addresses</ins> on
<ins>15 networks</ins> concurrently being called in 5 threads for a total of 1500 responses
per second.

The goal of this project is to be able to run it without hitting the CPU and
memory too much (which both python and javascript suffered through when I tried
to write it from those languages) so I can run the program 24/7.


<a id="installation"></a>

## Installation

You can do any of the following below:


### Install the Binary (recommended)

Grab the latest release from GitHub:

-   Install the binary from [Release](https://github.com/Sheape/metamask-bruteforce)


### Using cargo

-   Run `cargo install --git https://github.com/Sheape/metamask-bruteforce` from
    your terminal


### Compiling from source

Alternatively, you can clone the repo and build it manually from source:

    git clone https://github.com/Sheape/metamask-bruteforce.git
    cd metamask-bruteforce
    cargo build --release
    mv .env.template .env
    sudo cp ./target/release/metamask-bruteforce /usr/bin/metamask-bruteforce


## Usage

1.  [Install the package](#installation)
2.  Rename `.env.template` to `.env` or create one if you've installed from binary.
3.  Create an account to each of the following sites:
    -   [BscScan](https://bscscan.com/register)
    -   [Etherscan](https://etherscan.io/register)
    -   [Polygon Scan](https://polygonscan.com/register)
    -   [Polygon-zkEVM Scan](https://zkevm.polygonscan.com/register)
    -   [Arbiscan](https://arbiscan.io/register)
    -   [FTM Scan](https://ftmscan.com/register)
    -   [Optimism](https://optimistic.etherscan.io/register)
    -   [Cronoscan](https://cronoscan.com/register)
    -   [BitTorrent Scan](https://bttcscan.com/register)
    -   [Moonbeam](https://moonbeam.moonscan.io/register)
    -   [Moonriver](https://moonriver.moonscan.io/register)
    -   [Avalanche](https://snowtrace.io/register)
    -   [Celo Scan](https://celoscan.io/register)
    -   [Boba Scan](https://bobascan.com/register)
    -   [Gnosis Scan](https://gnosisscan.io/register)
4.  Create an API key for each of the network.
5.  Replace 'YOUR-API-KEY' to your api key.
6.  Run `metamask-bruteforce` from the terminal. Make sure `.env` is in the <ins>same
    directory</ins> as where you've installed it.
7.  If you found a wallet, it will be written in `found.txt` with the mnemonic,
    address, balance and which network it came from.

