use crate::globals;
use actix_web::{post, web::Json, HttpResponse};
use log::{error, warn};
use serde::{Deserialize, Serialize};

/// Pedido com dados criptografados com a chave RSA.
#[derive(Deserialize, Serialize)]
pub struct Request {
    data: Vec<u8>,
}

/// API /rsa/ que recebe uma mensagem criptografada, decifra a mensagem e printa na tela.
///
/// Erro: Atualmente não está funcionando corretamente. Não está conseguindo decifrar a mensagem recebida pelo front-end Javascript.
#[post("/rsa/")]
pub async fn read_rsa_message(req: Json<Request>) -> HttpResponse {
    // Desempacota o pedido do formato Json.
    let data: Vec<u8> = req.into_inner().data;
    // Informa a mensagem recebida.
    warn!("Requested data is: {data:?}");
    // Tenta decifrar: Atualmente causando um erro.
    let message = match globals::rsa_decrypt(&data) {
        Ok(msg) => msg,
        Err(err) => {
            error!("Error decrypting RSA message: {err:?}");
            return HttpResponse::BadRequest().body("Failed to decrypt message.".to_string());
        }
    };

    // Converte a mensagem decifrada para um string.
    let message = match String::from_utf8(message) {
        Ok(msg) => msg,
        Err(err) => {
            error!("Error parsing RSA message: {err:?}");
            return HttpResponse::BadRequest().body("Failed to parse message.".to_string());
        }
    };

    // Printa na tela a mensagem recebida com nivel de prioridade warn.
    warn!("RSA message received: {}", message);
    HttpResponse::Ok().finish()
}
