pub mod action;
pub mod cipher;
pub mod command;
pub mod config_file;
pub mod instructions;
pub mod utils;

use anyhow::Ok;
pub use anyhow::{Context, Result};
pub use cipher::*;
use console::style;

pub fn get_action(matches: &clap::ArgMatches) -> Result<action::Action> {
    let sub_m = |subcommand| -> Result<&clap::ArgMatches> {
        matches
            .subcommand_matches(subcommand)
            .context("args not found")
    };

    match matches.subcommand_name() {
        Some("config") => match sub_m("config")?.subcommand_name() {
            Some("init") => Ok(action::Action::InitConfig),
            Some("get") => Ok(action::Action::Get),
            Some("set") => {
                let matches = sub_m("config")?.subcommand_matches("set").unwrap();
                Ok(action::Action::Set {
                    rpc_url: matches.get_one::<String>("rpc_url").cloned(),
                    authority_path: matches.get_one::<String>("authority_path").cloned(),
                    approver_path: matches.get_one::<String>("approver_path").cloned(),
                })
            }
            _ => unreachable!(),
        },

        Some("escrow") => match sub_m("escrow")?.subcommand_name() {
            Some("get_config") => {
                let matches = sub_m("escrow")?.subcommand_matches("get_config").unwrap();
                Ok(action::Action::Escrow {
                    config_account: matches.get_one::<String>("account").cloned()
                })
            }
            _ => unreachable!(),
        },

        Some("encrypt") => {
            let sub_m = sub_m("encrypt")?;
            Ok(action::Action::Encrypt {
                private_key: sub_m
                    .get_one::<String>("private_key")
                    .expect("`--private_key` is required")
                    .to_owned(),
                password: sub_m
                    .get_one::<String>("password")
                    .expect("`--password` is required")
                    .to_owned(),
            })
        }
        Some("decrypt") => {
            let sub_m = sub_m("decrypt")?;
            Ok(action::Action::Decrypt {
                encrypted: sub_m
                    .get_one::<String>("encrypted")
                    .expect("`--encrypted` is required")
                    .to_owned(),
                password: sub_m
                    .get_one::<String>("password")
                    .expect("`--password` is required")
                    .to_owned(),
            })
        }
        _ => todo!(),
    }
}

fn main() {
    let cmd = command::new();
    let matches = cmd.get_matches();
    let action = get_action(&matches).unwrap();
    match action {
        action::Action::InitConfig { .. } => action::handler(action).unwrap(),
        action::Action::Get { .. } => {
            action::handler(action).unwrap();
        }
        action::Action::Set { .. } => {
            action::handler(action).unwrap();
        }

        action::Action::Escrow { .. } => {
            action::handler(action).unwrap();
        }

        action::Action::Encrypt { .. } => {
            action::handler(action).unwrap();
        }
        action::Action::Decrypt { .. } => {
            action::handler(action).unwrap();
        }
    }
}
