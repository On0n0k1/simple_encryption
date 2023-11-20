use crate::globals;
use actix_web::{post, web::Json, HttpResponse};
use aes_simple::CipherText;
use log::{error, warn};
use serde::{Deserialize, Serialize};

/// O formato do pedido recebido.
#[derive(Deserialize, Serialize)]
pub struct Request {
    data: Vec<u8>,
    nonce: Vec<u8>,
}

// Esta trait permite converter o pedido para CipherText
impl From<Request> for CipherText {
    fn from(req: Request) -> Self {
        CipherText {
            data: req.data,
            nonce: req.nonce,
        }
    }
}

/// Api /aes/ que recebe uma mensagem criptografada com a chave AES. Decifra e printa na tela com prioridade warn.
#[post("/aes/")]
pub async fn read_aes_message(req: Json<Request>) -> HttpResponse {
    let ciphertext: CipherText = req.into_inner().into();
    let decrypted: Vec<u8> = match globals::aes_decrypt(ciphertext) {
        Ok(decrypted) => decrypted,
        Err(err) => {
            error!("Error decrypting message: {err:?}");
            return HttpResponse::BadRequest().body("Failed to decrypt message.".to_string());
        }
    };

    let message: String = match String::from_utf8(decrypted) {
        Ok(message) => message,
        Err(err) => {
            error!("Error parsing decrypted message: {err:?}");
            return HttpResponse::BadRequest()
                .body("Failed to parse decrypted message.".to_string());
        }
    };
    warn!("AES message received: {message}");
    HttpResponse::Ok().finish()
}
