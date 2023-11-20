use aes_gcm::aead::{generic_array::GenericArray, Aead};
use aes_gcm::{Aes256Gcm, KeyInit};
use rand::Rng;

pub type Key = [u8; 32];
pub type Error = aes_gcm::Error;

/// Gera uma chave simétrica para criptografia aes_gcm.
pub fn new_key() -> Key {
    // array de 32 bytes inicializado com 0
    let mut key = [0u8; 32];
    // Substitui todos os elementos do array com números aleatórios.
    rand::thread_rng().fill(&mut key[..]);
    key
}

/// Cifra criptografada com protocolo aes. Contém os dados e o nonce utilizado para a criação desta cifra.
pub struct CipherText {
    pub data: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl CipherText {
    /// Cria uma nova cifra de acordo com uma fatia de bytes referenciada.
    pub fn new(key: &Key, plaintext: &[u8]) -> Result<Self, Error> {
        // Gera um nonce aleatório de 96 bits. Nunca utilize o mesmo nonce para criptografar dados diferentes com a mesma chave.
        let mut nonce = Vec::from([0u8; 12]);
        rand::thread_rng().fill(&mut nonce[..]);
        // Cria uma instância AES-GCM.
        let cipher: Aes256Gcm = Aes256Gcm::new(GenericArray::from_slice(key));
        // Criptografa os dados.
        let data: Vec<u8> = cipher.encrypt(GenericArray::from_slice(&nonce), plaintext.as_ref())?;
        let ciphertext: CipherText = CipherText { data, nonce };
        Ok(ciphertext)
    }

    /// Decifra esta cifra, retornando os bytes originais.
    pub fn decrypt(&self, key: &Key) -> Result<Vec<u8>, Error> {
        // Cria uma instância AES-GCM
        let cipher: Aes256Gcm = Aes256Gcm::new(GenericArray::from_slice(key));
        // Decifra os dados
        cipher.decrypt(GenericArray::from_slice(&self.nonce), &self.data[..])
    }
}

// Teste unitário
// cfg(test) significa que o módulo tests apenas existe durante testes.
#[cfg(test)]
mod tests {
    use super::*;

    // #[test] marca esta função como um teste unitário. Se ocorrer um erro, o teste falhou.
    #[test]
    fn encryption_decryption() {
        let key: Key = new_key();
        let plaintext: &[u8] = b"Testing encryption decryption";
        let ciphertext: CipherText = CipherText::new(&key, plaintext).unwrap();
        let decrypted: Vec<u8> = ciphertext.decrypt(&key).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
