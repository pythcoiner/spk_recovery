pub mod sync;
pub mod sign;
pub mod broadcast;

use miniscript::bitcoin::Amount;
use serde::{Deserialize, Serialize};
use miniscript::bitcoin::{OutPoint, ScriptBuf, TxOut};

#[derive(Debug, Clone)]
pub struct SyncResult {
    pub psbt: String,
    pub num_inputs: usize,
    pub total_value: Amount,
    pub fees: Amount,
    pub output_value: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpkEntry {
    pub spk: ScriptBuf,
    pub change: bool,
    pub index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coin {
    pub outpoint: OutPoint,
    pub txout: TxOut,
    pub value: Amount,
    pub spent: bool,
    pub is_change: bool,
    pub index: u32,
}
