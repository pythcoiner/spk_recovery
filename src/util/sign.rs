use std::str::FromStr;

use bwk::signer::HotSigner;
use miniscript::{
    bitcoin::{Network, Psbt},
    psbt::PsbtExt,
    Descriptor, DescriptorPublicKey,
};

pub fn sign_psbt(
    mnemonic: String,
    psbt_str: String,
    descriptor_str: String,
) -> Result<String, String> {
    println!("\n=== Starting PSBT Signing ===");

    let mut psbt = Psbt::from_str(&psbt_str)
        .map_err(|e| {
            println!("ERROR: Invalid PSBT: {}", e);
            format!("Invalid PSBT: {}", e)
        })?;

    println!("✓ PSBT parsed successfully, {} inputs", psbt.inputs.len());

    let mut signer = HotSigner::new_from_mnemonics(Network::Bitcoin, &mnemonic)
        .map_err(|e| {
            println!("ERROR: Failed to create signer: {:?}", e);
            format!("Failed to create signer from mnemonic: {:?}", e)
        })?;

    println!("✓ HotSigner created successfully");

    let descriptor_str = descriptor_str.trim();
    if descriptor_str.is_empty() {
        println!("ERROR: No descriptor provided");
        return Err("Descriptor is required for signing".to_string());
    }

    println!("Using descriptor: {}", descriptor_str);

    let descriptor = Descriptor::<DescriptorPublicKey>::from_str(descriptor_str)
        .map_err(|e| {
            println!("ERROR: Failed to parse descriptor: {}", e);
            format!("Invalid descriptor: {}", e)
        })?;

    println!("✓ Descriptor parsed");
    println!("Attempting to sign PSBT...");

    signer.inner_sign(&mut psbt, &descriptor)
        .map_err(|e| {
            println!("ERROR: Failed to sign PSBT: {:?}", e);
            format!("Failed to sign PSBT: {:?}", e)
        })?;

    println!("✓ PSBT signed successfully");

    println!("Finalizing PSBT...");
    let secp = miniscript::bitcoin::secp256k1::Secp256k1::new();
    psbt.finalize_mut(&secp)
        .map_err(|e| {
            println!("ERROR: Failed to finalize PSBT: {:?}", e);
            format!("Failed to finalize PSBT: {:?}", e)
        })?;

    println!("✓ PSBT finalized successfully");
    println!("=== Signing Complete ===\n");

    Ok(psbt.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_psbt_success() {
        let mnemonic = "".to_string();
        let psbt = "".to_string();
        let descriptor = "".to_string();

        let result = sign_psbt(mnemonic, psbt, descriptor);
        assert!(result.is_ok());
        let signed_psbt = result.unwrap();
        assert!(!signed_psbt.is_empty());
    }
}
