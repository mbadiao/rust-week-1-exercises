// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    let decode = hex::decode(raw_tx_hex).map_err(|_| "Hex decode error".to_string())?;

    if decode.len() < 4 {
        return Err("Transaction data too short".into());
    }
    let version = u32::from(decode[0]);
    Ok(version)
}
