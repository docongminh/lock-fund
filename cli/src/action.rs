use anyhow::Result;

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

pub fn handler(action: Action) -> Result<()> {
    match action {
        Action::Encrypt {
            private_key,
            password,
        } => {}
        Action::Decrypt {
            encrypted,
            password,
        } => {}
        Action::TransferToken {} => {}
        Action::TransferSol {} => {}
    }

    Ok(())
}
