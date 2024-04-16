
use ic_cdk::api::management_canister::bitcoin::GetUtxosResponse;
use ic_cdk::api::management_canister::bitcoin::MillisatoshiPerByte;
use ic_cdk::init;
use ic_cdk::print;
use ic_cdk::{api::management_canister::bitcoin::BitcoinNetwork, caller, update};
use std::cell::Cell;
use std::cell::RefCell;
use types::LaunchPadError;
use types::RuneInfo;

// use ic_cdk::{caller, update};
mod bitcoin_api;
mod bitcoin_wallet;
mod ecdsa_api;
mod types;

thread_local! {
    pub static NETWORK: Cell<BitcoinNetwork> = Cell::new(BitcoinNetwork::Regtest);
    // The ECDSA key name.
    static KEY_NAME: RefCell<String> = RefCell::new(String::from(""));
}

#[ic_cdk::update]
async fn withdraw_btc(dst_address : String, amount : u64) -> Result<String, LaunchPadError> {
    let network = NETWORK.with(|n| n.get());
    let derivation_path = vec![caller().to_text().as_bytes().to_vec()];
    let key_name = match network {
        BitcoinNetwork::Mainnet => "key_1",
        BitcoinNetwork::Testnet => "test_key_1",
        BitcoinNetwork::Regtest => "dfx_test_key",
    };
   
    let tx = bitcoin_wallet::withdraw(network, derivation_path, key_name.to_string(), dst_address, amount)
    .await;
    Ok(tx.to_string())
}

#[ic_cdk::update]
async fn etch_rune(info: RuneInfo) -> Result<String, LaunchPadError> {
    let network = NETWORK.with(|n| n.get());
    let derivation_path = vec![caller().to_text().as_bytes().to_vec()];
    let key_name = match network {
        BitcoinNetwork::Mainnet => "key_1",
        BitcoinNetwork::Testnet => "test_key_1",
        BitcoinNetwork::Regtest => "dfx_test_key",
    };
    let tx = bitcoin_wallet::etch_rune(
        network,
        derivation_path,
        key_name.to_string(),
        get_p2pkh_address().await,
        info,
    )
    .await;
    Ok(tx.to_string())
}

/// Returns the balance of the given bitcoin address.
#[update]
pub async fn get_balance(address: String) -> u64 {
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_balance(network, address).await
}

/// Returns the UTXOs of the given bitcoin address.
#[update]
pub async fn get_utxos(address: String) -> GetUtxosResponse {
    let network = NETWORK.with(|n| n.get());
    return bitcoin_api::get_utxos(network, address).await;
}

/// Returns the 100 fee percentiles measured in millisatoshi/byte.
/// Percentiles are computed from the last 10,000 transactions (if available).
#[update]
pub async fn get_current_fee_percentiles() -> Vec<MillisatoshiPerByte> {
    let network = NETWORK.with(|n| n.get());
    bitcoin_api::get_current_fee_percentiles(network).await
}

#[init]
pub fn init(network_opt: Option<BitcoinNetwork>) {
    let mut network = Some(BitcoinNetwork::Regtest);
    if network_opt.is_some() {
        network = network_opt
    }
    NETWORK.with(|n| n.set(network.unwrap()));

    KEY_NAME.with(|key_name| {
        key_name.replace(String::from(match network.unwrap() {
            // For local development, we use a special test key with dfx.
            BitcoinNetwork::Regtest => "dfx_test_key",
            // On the IC we're using a test ECDSA key.
            BitcoinNetwork::Mainnet | BitcoinNetwork::Testnet => "test_key_1",
        }))
    });
}

#[update]
async fn get_p2pkh_address() -> String {
    let derivation_path = vec![caller().to_text().as_bytes().to_vec()];
    let network: BitcoinNetwork = NETWORK.with(|d| d.get());


    let key_name = match network {
        BitcoinNetwork::Mainnet => "key_1",
        BitcoinNetwork::Testnet => "test_key_1",
        BitcoinNetwork::Regtest => "dfx_test_key",
    };

    print(key_name);

    bitcoin_wallet::get_p2pkh_address(network, key_name.to_string(), derivation_path).await
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

ic_cdk::export_candid!();
