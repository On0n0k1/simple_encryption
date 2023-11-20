use rand::rngs::ThreadRng;
use rsa::{Error, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

/// Cria uma nova chave para criptografia RSA com padding do tipo PKCS-1v.15
pub fn new_keys(rng: &mut ThreadRng) -> Result<(RsaPrivateKey, RsaPublicKey), Error> {
    // Tamanho da chave em bits
    let bits: usize = 2048;
    // Chave privada
    let private_key: RsaPrivateKey = RsaPrivateKey::new(rng, bits)?;
    // Chave publica
    let public_key: RsaPublicKey = RsaPublicKey::from(&private_key);
    Ok((private_key, public_key))
}

/// Criptografa uma fatia de dados de acordo com uma fatia de bytes.
pub fn encrypt(
    // Uma instância para geração de números aleatórios
    rng: &mut ThreadRng,
    public_key: &RsaPublicKey,
    msg: &[u8],
) -> Result<Vec<u8>, Error> {
    public_key.encrypt(rng, Pkcs1v15Encrypt, msg)
}

/// Decifra uma fatia de bytes utilizando a chave privada.
pub fn decrypt(private_key: &RsaPrivateKey, ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    private_key.decrypt(Pkcs1v15Encrypt, ciphertext)
}

// Teste unitário
#[cfg(test)]
mod tests {
    use super::*;

    /// Testa se cifrando e decifrando resulta no mesmo dado.
    ///
    /// Execute este teste no diretório raiz deste projeto com o comando `cargo test --release -p rsa_simple`
    #[test]
    fn encrypt_decrypt() {
        let mut rng = rand::thread_rng();
        // o b significa que isto está no formato de bytes
        let plaintext = b"Test plaintext 1234567";
        let (private_key, public_key) = new_keys(&mut rng).unwrap();
        // criptografa
        let encrypted: Vec<u8> = encrypt(&mut rng, &public_key, plaintext).unwrap();
        // decifra
        let decrypted: Vec<u8> = decrypt(&private_key, &encrypted).unwrap();
        // Converte os bytes para uma String
        let decrypted: String = String::from_utf8(decrypted).unwrap();
        assert_eq!(
            String::from_utf8(plaintext[..].to_owned()).unwrap(),
            decrypted,
            "Encryption Decryption failed"
        );
    }
}
