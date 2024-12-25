// Use to loading and saving cli config file
// cli config file includes info about rpc, authority, approver signers program instructions

use crate::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct ConfigFile {
    pub rpc_url: String,
    pub wss_url: String,
    pub approver_path: String,
    pub authority_path: String,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            rpc_url: String::from("https://api.mainnet-beta.solana.com"),
            wss_url: String::from(""),
            authority_path: String::from(""),
            approver_path: String::from(""),
        }
    }
}

impl ConfigFile {
    pub fn load_config(file: &str) -> Result<Self, std::io::Error> {
        crate::utils::load_from_file(file)
    }

    pub fn save_config(&self, file: &str) -> Result<(), std::io::Error> {
        crate::utils::save_to_file(self, file)
    }
}
