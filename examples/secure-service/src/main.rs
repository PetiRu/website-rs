fn main() -> Result<(), website_encryption::Error> {
    let sealer = website_encryption::Sealer::new(website_encryption::Key::from_bytes([42; 32]));
    let token = sealer.seal(b"encrypted website data", b"session:v1")?;
    println!("encrypted {} bytes; decrypted: {}", token.len(), String::from_utf8_lossy(&sealer.open(&token, b"session:v1")?));
    Ok(())
}
