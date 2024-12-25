use serde_json;
use std::{
    fs::{self, create_dir_all},
    io::{self, Write},
    path::Path,
};

pub fn load_from_file<T, P>(config_file: P) -> Result<T, io::Error>
where
    T: serde::de::DeserializeOwned,
    P: AsRef<Path>,
{
    let file = fs::File::open(config_file)?;
    let config = serde_json::from_reader(file)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{err:?}")))?;

    Ok(config)
}

pub fn save_to_file<T, P>(config: &T, config_file: P) -> Result<(), io::Error>
where
    T: serde::ser::Serialize,
    P: AsRef<Path>,
{
    let serialized = serde_json::to_string(config)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{err:?}")))?;

    if let Some(outdir) = config_file.as_ref().parent() {
        create_dir_all(outdir)?;
    }
    let mut file = fs::File::create(config_file)?;
    file.write_all(&serialized.into_bytes())?;

    Ok(())
}
