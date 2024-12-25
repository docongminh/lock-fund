use std::{path::Path, process::exit};

use anyhow::Result;
use config_file::ConfigFile;
use sha2::{Digest, Sha256};
use utils::{load_from_file, save_to_file};

use crate::*;

#[macro_export]
macro_rules! config_path {
    () => {{
        let mut root_path = dirs_next::home_dir().expect("home directory");
        root_path.extend([".config", ".lock-fund-cli", "config.json"]);
        root_path.to_str().unwrap().to_string()
    }};
}

#[derive(Debug)]
pub enum Action {
    InitConfig,
    Get,
    Set {
        rpc_url: Option<String>,
        authority_path: Option<String>,
        approver_path: Option<String>,
    },
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

// hash password to 32 bytes to feed into Generic Array
pub fn hash_to_32_bytes(input: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().into()
}

// Pretty print a "name value"
pub fn println_name_value(name: &str, value: &str) {
    let styled_value = if value.is_empty() {
        style("(not set)").italic()
    } else {
        style(value)
    };
    println!("{} {}", style(name).bold(), styled_value);
}

pub fn handler(action: Action) -> Result<()> {
    match action {
        Action::InitConfig => {
            let default = ConfigFile::default();
            let root_file_path = config_path!();

            if Path::new(&root_file_path).exists() {
                println_name_value("Config file existed at: ", &root_file_path);
                exit(1);
            }

            save_to_file(&default, root_file_path.clone())?;

            println_name_value("Config file created at: ", &root_file_path);
        }
        Action::Get => {
            let file_path = config_path!();
            if !Path::new(&file_path).exists() {
                println_name_value("Not found config file at: ", &file_path);
                exit(1);
            }
            let config =
                load_from_file::<ConfigFile, String>(file_path.clone()).expect("Config file created");
                println_name_value("FILE PATH: ", &file_path);   
            println_name_value("RPC URL: ", &config.rpc_url);
            println_name_value("WebSocket URL: ", &config.wss_url);
            println_name_value("Approver Path: ", &config.approver_path);
            println_name_value("Authority Path: ", &config.authority_path);
        }
        Action::Set {
            rpc_url,
            authority_path,
            approver_path,
        } => {
            let file_path = config_path!();
            if !Path::new(&file_path).exists() {
                println_name_value("Not found config file at: ", &file_path);
                exit(1);
            }

            let mut config = load_from_file::<ConfigFile, String>(file_path.clone())
                .expect("Config file created");
            if let Some(rpc_url) = rpc_url {
                config.rpc_url = rpc_url;
            }
            if let Some(authority_path) = authority_path {
                config.authority_path = authority_path;
            }

            if let Some(approver_path) = approver_path {
                config.approver_path = approver_path;
            }

            save_to_file(&config, file_path)?;
        }
        Action::Encrypt {
            private_key,
            password,
        } => {
            let private_key_data = load_from_file::<Vec<u8>, String>(private_key).unwrap();
            let to_32_bytes = hash_to_32_bytes(&password);
            let encrypted_data =
                cipher::encrypt(private_key_data.as_slice(), &to_32_bytes, None).unwrap();

            save_to_file::<Vec<u8>, String>(&encrypted_data, String::from("encrypt.json"))?;
        }
        Action::Decrypt {
            encrypted,
            password,
        } => {
            let encrypted_data: Vec<u8> = load_from_file::<Vec<u8>, String>(encrypted).unwrap();
            let to_32_bytes = hash_to_32_bytes(&password);
            let private_key =
                cipher::decrypt(&encrypted_data.as_slice(), &to_32_bytes, None).unwrap();

            save_to_file::<Vec<u8>, String>(
                &private_key,
                String::from("decrypt_private_Key.json"),
            )?;
        }
        Action::TransferToken {} => {}
        Action::TransferSol {} => {}
    }

    Ok(())
}
