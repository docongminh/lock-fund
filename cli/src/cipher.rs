use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng, Payload},
    Aes256Gcm,
};
use anyhow::{anyhow, Result};

pub fn encrypt(private_key: &[u8], password: &[u8], aad: Option<&[u8]>) -> Result<Vec<u8>> {
    let pass_key: &GenericArray<u8, _> = GenericArray::<u8, _>::from_slice(password);
    let cipher = Aes256Gcm::new(pass_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let aad_option = match aad {
        Some(aad) => aad,
        _ => &[],
    };
    let payload = Payload {
        msg: private_key,
        aad: aad_option,
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

pub fn decrypt(
    encrypted_private_key: &[u8],
    password: &[u8],
    aad: Option<&[u8]>,
) -> Result<Vec<u8>> {
    let pass_key = GenericArray::from_slice(password);
    let cipher = Aes256Gcm::new(pass_key);
    let nonce = GenericArray::from_slice(&encrypted_private_key[..12]);
    let ciphertext = &encrypted_private_key[12..];
    let aad_option = match aad {
        Some(aad) => aad,
        _ => &[],
    };
    let payload = Payload {
        msg: ciphertext,
        aad: aad_option,
    };

    cipher
        .decrypt(nonce, payload)
        .map_or_else(|_| Err(anyhow!("Failed to decrypt data")), Ok)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_client::solana_sdk::signer::keypair::Keypair;

    const AAD: [u8; 32] = [1_u8; 32];

    #[test]
    fn test_with_aad() {
        let keypair = Keypair::new();
        let private_key = keypair.secret().as_bytes();

        let password = [0_u8; 32];
        let encrypted = encrypt(private_key, &password, Some(&AAD)).unwrap();

        let decrypted = decrypt(&encrypted, &password, Some(&AAD)).unwrap();

        assert_eq!(private_key.as_ref(), decrypted);
    }

    #[test]
    fn test_without_aad() {
        let keypair = Keypair::new();
        let private_key = keypair.secret().as_bytes();

        let password = [0_u8; 32];
        let encrypted = encrypt(private_key, &password, None).unwrap();

        let decrypted = decrypt(&encrypted, &password, None).unwrap();

        assert_eq!(private_key.as_ref(), decrypted);
    }
}
