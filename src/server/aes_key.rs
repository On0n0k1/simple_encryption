use crate::globals;
use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};

/// A Chave Aes.
#[derive(Deserialize, Serialize)]
pub struct AESResponse {
    key: [u8; 32],
}

/// Uma mensagem para o api /aes/key retorna a chave AES.
#[get("/aes/key")]
pub async fn get_aes_key() -> HttpResponse {
    let key = globals::get_aes_key();
    let response = AESResponse { key };
    HttpResponse::Ok().json(&response)
}
