pub mod action;
pub mod cipher;
pub mod command;
pub mod instructions;

pub use anyhow::{Context, Result};
pub use cipher::*;

pub fn get_action(matches: &clap::ArgMatches) -> Result<action::Action> {
    let sub_m = |subcommand| -> Result<&clap::ArgMatches> {
        matches
            .subcommand_matches(subcommand)
            .context("args not found")
    };

    match matches.subcommand_name() {
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
        action::Action::Encrypt { .. } => {
            action::handler(action).unwrap();
        }
        action::Action::Decrypt { .. } => {
            action::handler(action).unwrap();
        }
        action::Action::TransferToken { .. } => {
            action::handler(action).unwrap();
        }
        action::Action::TransferSol { .. } => {
            action::handler(action).unwrap();
        }
    }
}
