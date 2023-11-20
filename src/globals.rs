//! Para simplificar o acesso a memória, as chaves são armazenadas na memória global durante inicialização.
//! Este módulo possui funções para gerar e acessar as chaves AES e RSA.

use once_cell::sync::Lazy;
use rand::rngs::ThreadRng;
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::sync::{Arc, Mutex};

/// Chaves RSA pública e privada.
pub struct RSAKeys {
    private: RsaPrivateKey,
    public: RsaPublicKey,
}

impl RSAKeys {
    /// Retorna uma referência da chave privada sem necessitar de cópia.
    pub fn get_private_key(&self) -> &RsaPrivateKey {
        &self.private
    }

    /// Retorna uma referência da chave pública sem necessitar de cópia.
    pub fn get_public_key(&self) -> &RsaPublicKey {
        &self.public
    }
}

// Trait para criar uma instância de RSAKeys sem necessitar de argumentos.
impl Default for RSAKeys {
    fn default() -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let (private, public) = rsa_simple::new_keys(&mut rng).unwrap();
        RSAKeys { private, public }
    }
}

// Variáveis globais:
// Lazy é um tipo de dado da crate once_cell que é criado dinamicamente. Ele desativa checagens e erros de compilação.
// Arc é um ponteiro atômico de memória. Não permite modificar o dado. Mas permitem várias threads acessarem o mesmo dado.
// Mutex é um ponteiro que permite modificar o dado interno. Mas cada thread precisa "trancar" (lock) o dado antes de acessar ou modificar.

pub static RSA_KEYS: Lazy<Arc<Mutex<RSAKeys>>> =
    Lazy::new(|| Arc::new(Mutex::new(RSAKeys::default())));

pub static AES_KEY: Lazy<Arc<Mutex<[u8; 32]>>> =
    Lazy::new(|| Arc::new(Mutex::new(aes_simple::new_key())));

/// Cria uma cópia da chave privada RSA global e retorna.
pub fn get_rsa_private_key() -> RsaPrivateKey {
    RSA_KEYS.lock().unwrap().get_private_key().clone()
}

/// Cria uma cópia da chave pública RSA e retorna.
pub fn get_rsa_public_key() -> RsaPublicKey {
    RSA_KEYS.lock().unwrap().get_public_key().clone()
}

/// Cria uma cópia da chave AES e retorna.
pub fn get_aes_key() -> [u8; 32] {
    *AES_KEY.lock().unwrap()
}

/// Carrega a chave AES e criptografa uma fatia de bytes.
pub fn aes_encrypt(plaintext: &[u8]) -> Result<aes_simple::CipherText, aes_simple::Error> {
    let key: aes_simple::Key = get_aes_key();
    aes_simple::CipherText::new(&key, plaintext)
}

/// Carrega a chave AES e decifra uma cifra AES.
pub fn aes_decrypt(ciphertext: aes_simple::CipherText) -> Result<Vec<u8>, aes_simple::Error> {
    let key: aes_simple::Key = get_aes_key();
    ciphertext.decrypt(&key)
}

/// Carrega a chave pública RSA e criptografa uma fatia de bytes.
pub fn rsa_encrypt(rng: &mut ThreadRng, msg: &[u8]) -> Result<Vec<u8>, rsa::Error> {
    let public_key: RsaPublicKey = get_rsa_public_key();
    rsa_simple::encrypt(rng, &public_key, msg)
}

/// Carrega a chave privada RSA e descriptografa uma fatia de bytes.
pub fn rsa_decrypt(ciphertext: &[u8]) -> Result<Vec<u8>, rsa::Error> {
    let private_key: RsaPrivateKey = get_rsa_private_key();
    rsa_simple::decrypt(&private_key, ciphertext)
}
