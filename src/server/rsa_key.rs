use actix_web::{get, HttpResponse};
use log::warn;
use rsa::{traits::PublicKeyParts, BigUint, RsaPublicKey};
use serde::{Deserialize, Serialize};

use crate::globals;

/// Resposta da chave RSA.
///
/// e é um número primo
/// n é um big integer composto por 2048 bits ou 64 numeros de 32 bits.
#[derive(Serialize, Deserialize)]
pub struct RSAResponse {
    e: String,
    n: String,
}

/// API /rsa/key que retorna a chave pública do protocolo RSA.
#[get("/rsa/key")]
pub async fn get_rsa_public_key() -> HttpResponse {
    let public_key: RsaPublicKey = globals::get_rsa_public_key();
    let e: BigUint = public_key.e().clone();
    let n: BigUint = public_key.n().clone();
    warn!("e is {e}");
    warn!("n is {n}");
    let e: String = format!("{e}");
    let n: String = format!("{n}");

    let response = RSAResponse { e, n };
    HttpResponse::Ok().json(response)
}
