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
    // Usage: cargo run -- encode OR cargo run -- decode
    // let args: Vec<String> = env::args().collect();

    encode()?;
    Ok(())
}

/// Reads the password from the file "pswd"
fn read_password() -> Result<String, Box< dyn std::error::Error>> {
    let mut file = File::open("pswd")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents.trim().to_string())
}

#[allow(dead_code)]
fn encode() -> Result<(), Box<dyn std::error::Error>> {
    let password = read_password()?;
    // println!("You placed the \"{}\" password in the file", password);

    let mut source = File::open("source_file")?;
    let mut plaintext = Vec::new();
    source.read_to_end(&mut plaintext)?;

    let mut salt = [0u8; 16];
    // let mut rng = OsRng::default();
    OsRng.fill_bytes(&mut salt);

    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    
    argon2
        .hash_password_into(password.as_bytes(), &salt, &mut key)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    // println!("Generated key: {:?}", &key);

    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);

    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));

    let ciphertext = cipher.encrypt(GenericArray::from_slice(&nonce), plaintext.as_ref())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        // .expect("encryption failure!");

    let mut out_file = File::create("box")?;
    out_file.write_all(&salt)?;
    out_file.write_all(&nonce)?;
    out_file.write_all(&ciphertext)?;

    println!("Encryption successful. Encrypted data stored in 'box'.");

    Ok(())
}

#[allow(dead_code)]
fn decode() -> Result<(), Box<dyn std::error::Error>> {
    let password = read_password()?;

    let mut file = File::open("box")?;
    let mut data = Vec::new();
}