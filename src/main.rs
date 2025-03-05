use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

use rand::rngs::OsRng;
use rand::RngCore;
use argon2::Argon2;

use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use aes_gcm::Aes256Gcm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // encode()?;
    decode()?;
    Ok(())
}

/// Reads the password from the file "pswd"
fn read_password() -> Result<String, Box< dyn std::error::Error>> {
    let mut file = File::open("pswd")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents.trim().to_string())
}

fn generate_cipher(salt: &[u8]) -> Result<Aes256Gcm, Box< dyn std::error::Error>> {
    let password = read_password()?;

    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));

    Ok(cipher)
}

#[allow(dead_code)]
fn encode() -> Result<(), Box<dyn std::error::Error>> {
    let mut source = File::open("source_file")?;
    let mut plaintext = Vec::new();
    source.read_to_end(&mut plaintext)?;

    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    let cipher = generate_cipher(&salt)?;

    let ciphertext = cipher.encrypt(GenericArray::from_slice(&nonce), plaintext.as_ref())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let mut out_file = File::create("box")?;
    out_file.write_all(&salt)?;
    out_file.write_all(&nonce)?;
    out_file.write_all(&ciphertext)?;

    println!("Encryption successful. Encrypted data stored in 'box'.");

    Ok(())
}

#[allow(dead_code)]
fn decode() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("box")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    if data.len() < 28 {
        return Err("The 'box' file is too short to contain necessary encryption parameters.".into());
    }

    let salt = &data[0..16];
    let nonce = &data[16..28];
    let ciphertext = &data[28..];

    let cipher = generate_cipher(salt)?;

    let plaintext = cipher.decrypt(GenericArray::from_slice(nonce), ciphertext.as_ref())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    println!("{}", String::from_utf8_lossy(&plaintext));

    Ok(())
}