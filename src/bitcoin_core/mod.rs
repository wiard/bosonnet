pub mod bitcoin_core;
pub mod utxo_manager;
pub mod consensus;
pub mod blockchain;

pub fn init_bitcoin_core() {
    println!("Bitcoin Core layer initialized.");
    bitcoin_core::connect_to_bitcoin();
}

