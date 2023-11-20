//! Contém a implementação do servidor
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use log::info;
use std::fs;

// Declaração de módulos
pub mod globals;
pub mod server;

// É possivel criar uma única função que faz o que index, css, aes_js e rsa_js faz. Mas estava com pressa.

/// Serve o arquivo `/static/index.html` como tipo de conteúdo `text/html`.
async fn index() -> Result<HttpResponse> {
    let html_content = fs::read_to_string("static/index.html")
        .unwrap_or_else(|_| String::from("Error reading HTML file"));
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content))
}

/// Serve o arquivo `/static/style.css` como tipo de conteúdo `text/css`.
async fn css() -> Result<HttpResponse> {
    let css_content = fs::read_to_string("static/style.css")
        .unwrap_or_else(|_| String::from("Error reading CSS file"));
    Ok(HttpResponse::Ok()
        .content_type("text/css")
        .body(css_content))
}

/// Serve o arquivo `/static/aes.js` como tipo de conteúdo `application/javascript`.
async fn aes_js() -> Result<HttpResponse> {
    let js_content = fs::read_to_string("static/aes.js")
        .unwrap_or_else(|_| String::from("Error reading aes.js file"));
    Ok(HttpResponse::Ok()
        .content_type("application/javascript")
        .body(js_content))
}

/// Serve o arquivo `/static/rsa.js` como tipo de conteúdo `application/javascript`.
async fn rsa_js() -> Result<HttpResponse> {
    let js_content = fs::read_to_string("static/rsa.js")
        .unwrap_or_else(|_| String::from("Error reading rsa.js file"));
    Ok(HttpResponse::Ok()
        .content_type("application/javascript")
        .body(js_content))
}

/// Função principal que inicializa o log, algumas variaveis de ambiente, e o servidor.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "simple_encryption=info, actix_web=info");
    env_logger::init();
    let address = "0.0.0.0:8080";
    info!("Starting server on {address}");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/style.css").to(css))
            .service(web::resource("/aes.js").to(aes_js))
            .service(web::resource("/rsa.js").to(rsa_js))
            .service(server::get_aes_key)
            .service(server::get_rsa_public_key)
            .service(server::read_aes_message)
            .service(server::read_rsa_message)
    })
    .bind(address)?
    .run()
    .await
}
