use std::str::FromStr;

use miniscript::bitcoin::{Psbt, Txid};

pub fn broadcast_psbt(
    psbt_str: String,
    ip: String,
    port: String,
) -> Result<Txid, String> {
    let psbt = Psbt::from_str(&psbt_str)
        .map_err(|e| format!("Invalid PSBT: {}", e))?;

    let port: u16 = port.parse()
        .map_err(|e| format!("Invalid port: {}", e))?;

    let tx = psbt.extract_tx()
        .map_err(|e| format!("Failed to extract transaction: {}", e))?;
    let txid = tx.compute_txid();

    println!("Broadcasting transaction: {}", txid);

    let mut client = bwk_electrum::client::Client::new_local(&ip, port)
        .map_err(|e| format!("Failed to connect to Electrum server: {:?}", e))?;

    client.broadcast(&tx)
        .map_err(|e| format!("Failed to broadcast transaction: {:?}", e))?;

    println!("Transaction broadcast successfully: {}", txid);

    Ok(txid)
}
