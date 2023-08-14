

# Metamask Bruteforce

This is my first project written in rust, so please if you have any issues feel
free to open them or if you have suggestions on how I can improve my code.

![img](https://user-images.githubusercontent.com/86521166/260396429-eb877d79-82c3-4301-900d-cd99401fcde8.png)


## Features

-   **Low memory usage** - Only using about <span class="underline">20MB of RAM</span>, so you can run this without
    affecting your daily workflow.
-   **Minimum CPU usage** - Only one of spike of CPU every second. Unlike other options
    where hashing will inevitably impact your CPU.
-   **Realiable** - It uses an [online API](https://blockscan.com) to check the balance of the address instead
    of doing it offline where the user is required to install a very big database.
-   **Blazingly fast** - It maximizes the performance while scanning all of the
     generated address and still maintaining a low memory usage using <span class="underline">asynchronous
    code</span>.
-   **Binary-only**: Unlike other scripts where it is interpreted at runtime, since
    rust compiles a program to a binary, there is no need for you to install an
    interpreter or compiler.


## How it works?

This works by efficiently generating 12 words mnemonic and hashing it to an
ethereum address. 100 pairs are generated in a matter of `50ms` which is
extremely fast and only the network and API calls take much of the time.

It uses [Blockscan](https://blockscan.com/) APIs and its incredible ecosystem for each of the network as
they all have the same framework of API. Each API call process <span class="underline">20 addresses</span> on
<span class="underline">15 networks</span> concurrently being called in 5 threads for a total of 1500 responses
per second.

The goal of this project is to be able to run it without hitting the CPU and
memory too much (which both python and javascript suffered through when I tried
to write it from those languages) so I can run the program 24/7.


<a id="Installation"></a>

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

1.  [Install the package](#Installation)
2.  Rename `.env.template` to `.env`
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
5.  Replace &rsquo;YOUR-API-KEY&rsquo; to your api key.
6.  Run `metamask-bruteforce` from the terminal. Make sure `.env` is in the same
    directory as where you&rsquo;ve installed it.
7.  If you found a wallet, it will be written in `found.txt` with the mnemonic,
    address, balance and which network it came from.

