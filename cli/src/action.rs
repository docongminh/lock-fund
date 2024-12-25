use anchor_client::solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
};
use anyhow::Result;
use config_file::ConfigFile;
use instructions::{CreateConfigParams, InitProgramParams, LockFundProgram};
use sha2::{Digest, Sha256};
use std::{path::Path, process::exit, str::FromStr};
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

#[macro_export]
macro_rules! validate_keypair {
    ($path:expr) => {{
        let bytes_data = load_from_file::<Vec<u8>, String>($path).unwrap();
        let _ = Keypair::from_bytes(bytes_data.as_slice());
    }};
}

#[macro_export]
macro_rules! get_address {
    ($path:expr) => {{
        let bytes_data = load_from_file::<Vec<u8>, String>($path).unwrap();
        let keypair = Keypair::from_bytes(bytes_data.as_slice())?;

        bs58::encode(keypair.pubkey().to_bytes()).into_string()
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
    EscrowConfig {
        config_account: Option<String>,
    },
    InitEscrow {
        recipient: String,
    },
    Encrypt {
        private_key: String,
        password: String,
    },
    Decrypt {
        encrypted: String,
        password: String,
    },
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
    let file_path = config_path!();
    let config =
        load_from_file::<ConfigFile, String>(file_path.clone()).expect("Config file created");

    let params: InitProgramParams = InitProgramParams {
        rpc_url: config.rpc_url,
        wss_url: config.wss_url,
        authority_path: config.authority_path,
        approver_path: config.approver_path,
    };
    let program = LockFundProgram::init(params);
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
            let config = load_from_file::<ConfigFile, String>(file_path.clone())
                .expect("Config file created");
            let authority_address = get_address!(config.authority_path.clone());
            let approver_address = get_address!(config.approver_path.clone());
            println_name_value("FILE PATH: ", &file_path);
            println_name_value("RPC URL: ", &config.rpc_url);
            println_name_value("WebSocket URL: ", &config.wss_url);
            println_name_value("Approver Path: ", &config.approver_path);
            println_name_value("Authority Address: ", &authority_address);
            println_name_value("Authority Path: ", &config.authority_path);
            println_name_value("Approver Address: ", &approver_address);
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
                validate_keypair!(authority_path.clone());
                config.authority_path = authority_path;
            }

            if let Some(approver_path) = approver_path {
                validate_keypair!(approver_path.clone());
                config.approver_path = approver_path;
            }

            save_to_file(&config, file_path)?;
        }
        Action::EscrowConfig { config_account } => {
            let config_data: lock_fund::ConfigAccount = program.config_data(config_account)?;
            println_name_value("Authority: ", &config_data.authority.to_string());
            println_name_value("Approver: ", &config_data.approver.to_string());
            println_name_value("Recipient: ", &config_data.recipient.to_string());
            println_name_value(
                "Enable transfer full: ",
                &config_data.enable_transfer_full.to_string(),
            );
        }

        Action::InitEscrow { recipient } => {
            let params = CreateConfigParams {
                cliff_time_duration: 24 * 60 * 60,
                amount_per_day: 1_000_000,
                update_actor_mode: 0,
                enable_transfer_full: 0,
                recipient: Pubkey::from_str(&recipient)?,
                approver: program.approver.pubkey(),
            };
            let sig = program.create_config(params).unwrap();
            println_name_value("New escrow config created: ", &program.config_account.to_string());
            println_name_value("New escrow account created: ", &program.escrow.to_string());
            println_name_value(
                "Create escrow transaction: ",
                &bs58::encode(sig).into_string(),
            );
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
    }

    Ok(())
}
