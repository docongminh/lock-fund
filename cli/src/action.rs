use std::{fs, io::BufWriter};

use anyhow::Result;
use serde_json;
use sha2::{Sha256, Digest};

use crate::*;

#[derive(Debug)]
pub enum Action {
    Encrypt {
        private_key: String,
        password: String,
    },
    Decrypt {
        encrypted: String,
        password: String,
    },
    TransferToken {},
    TransferSol {},
}

pub fn hash_to_32_bytes(input: &str) -> [u8;32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}

pub fn handler(action: Action) -> Result<()> {
    match action {
        Action::Encrypt {
            private_key,
            password,
        } => {
            let file = fs::File::open(private_key).expect("file should open read only");
            let private_key_data: Vec<u8> =
                serde_json::de::from_reader(file).expect("file should be proper JSON");
            let to_32_bytes = hash_to_32_bytes(&password);
            let encrypted_data = cipher::encrypt(private_key_data.as_slice(), &to_32_bytes, None).unwrap();
            println!("encrypted private key: {:?}", encrypted_data);

            // write encrypted data to output
            let output = fs::File::create("encrypted.json").expect("Failed to create file output");
            let writer = BufWriter::new(output);
            serde_json::to_writer(writer, &encrypted_data)?;

        }
        Action::Decrypt {
            encrypted,
            password,
        } => {
            let file = fs::File::open(encrypted).expect("file should open read only");
            let encrypted_data: Vec<u8> =
                serde_json::de::from_reader(file).expect("file should be proper JSON");
            let to_32_bytes = hash_to_32_bytes(&password);
            let private_key = cipher::decrypt(&encrypted_data.as_slice(), &to_32_bytes, None).unwrap();
            println!("private key: {:?}", private_key);

            // write encrypted data to output
            let output = fs::File::create("decrypted.json").expect("Failed to create file output");
            let writer = BufWriter::new(output);
            serde_json::to_writer(writer, &private_key)?;
        }
        Action::TransferToken {} => {}
        Action::TransferSol {} => {}
    }

    Ok(())
}
