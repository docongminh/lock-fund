use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng, Payload},
    Aes256Gcm,
};
use anyhow::{anyhow, Result};
use secrecy::{ExposeSecret, SecretSlice};

pub struct Cipher {
    pub key: &[u8],
}

impl Cipher {
    pub fn encrypt(self, password: &str) -> Result<Vec<u8>> {
        let key = GenericArray::from_slice(self.key.expose_secret());
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let payload = Payload {
            msg: data,
            aad: fingerprint,
        };

        cipher.encrypt(&nonce, payload).map_or_else(
            |_| Err(anyhow!("Failed to encrypt data")),
            |ciphertext| {
                let mut encrypted_data = nonce.to_vec();
                encrypted_data.extend_from_slice(&ciphertext);
                Ok(encrypted_data)
            },
        )
    }

    fn decrypt(&self, data: &[u8], fingerprint: &[u8]) -> Result<Vec<u8>> {
        let key = GenericArray::from_slice(self.key.expose_secret());
        let cipher = Aes256Gcm::new(key);
        let nonce = GenericArray::from_slice(&data[..12]);
        let ciphertext = &data[12..];
        let payload = Payload {
            msg: ciphertext,
            aad: fingerprint,
        };

        cipher
            .decrypt(nonce, payload)
            .map_or_else(|_| Err(anyhow!("Failed to decrypt data")), Ok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "The quick brown fox jumps over the lazy dog";
    const FINGERPRINT: &str = "SHA256:hgIL5fEHz5zuOWY1CDlUuotdaUl4MvYG7vAgE4q4TzM";

}
