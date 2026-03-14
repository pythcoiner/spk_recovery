pub mod broadcast;
pub mod sign;
pub mod sync;

use miniscript::bitcoin::Amount;

#[derive(Debug, Clone)]
pub struct SyncResult {
    pub psbt: String,
    pub num_inputs: usize,
    pub total_value: Amount,
    pub fees: Amount,
    pub output_value: Amount,
}
