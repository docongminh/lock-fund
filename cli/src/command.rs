use clap::{
    builder::styling::{AnsiColor, Effects, Styles},
    Arg, Command,
};

pub fn new() -> Command {
    let styles = Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default());

    Command::new("lock-cli")
        .about("encrypt/decrypt private key and interact with lock fund program")
        .color(clap::ColorChoice::Auto)
        .styles(styles)
        .subcommand(command_encrypt())
        .subcommand(command_decrypt())
        .subcommand(command_transfer_token())
        .subcommand(command_transfer_sol())
}

pub fn command_encrypt() -> Command {
    Command::new("encrypt")
        .about("Encrypt private key with password")
        .arg(
            Arg::new("private_key")
                .short('k')
                .long("private_key")
                .help("Private key using to encrypt"),
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .help("Password using to encrypt"),
        )
}

pub fn command_decrypt() -> Command {
    Command::new("decrypt")
        .about("Decrypt encrypted private key with password")
        .arg(
            Arg::new("encrypted")
                .short('k')
                .long("private_key")
                .help("encrypted private key ussing to decrypt"),
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .help("Password using to decrypt"),
        )
}

pub fn command_transfer_token() -> Command {
    Command::new("transfer_token")
        .about("Transfer token from vault to recipient")
        .arg(
            Arg::new("encrypted")
                .short('k')
                .long("private_key")
                .help("encrypted private key ussing to decrypt"),
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .help("Password using to decrypt"),
        )
}

pub fn command_transfer_sol() -> Command {
    Command::new("transfer_sol")
        .about("Transfer SOL from vault to recipient")
        .arg(
            Arg::new("encrypted")
                .short('k')
                .long("private_key")
                .help("encrypted private key ussing to decrypt"),
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .help("Password using to decrypt"),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let command = new();

        assert_eq!(command.get_name(), "lock-cli");
        assert_eq!(
            command.get_about().unwrap().to_string(),
            "encrypt/decrypt private key and interact with lock fund program"
        );
    }
}