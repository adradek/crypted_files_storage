use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process;

use rand::rngs::OsRng;
use rand::RngCore;
use argon2::Argon2;

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
    println!("You placed the \"{}\" password in the file", password);

    Ok(())
}

#[allow(dead_code)]
fn decode() {
    
}