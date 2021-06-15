// System miodules
use ::config::ConfigError;
use dotenv::dotenv;

// jSON data handling
use serde::{Deserialize, Serialize};

// Actix related
use actix_web::{web, App, http, HttpServer, Result};
use actix_cors::Cors;

// Custom built modules
mod handlers;

// To parse env params
#[derive(Debug, Deserialize)]
struct EnvConfig {
    pub server_addr: String,
}

impl EnvConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = ::config::Config::new();
        cfg.merge(::config::Environment::new())?;
        cfg.try_into()
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Env details - Only for dev and testing
    dotenv().ok();
    let config = EnvConfig::from_env().unwrap();

    //TODO: Add a route for predicting
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                  //TODO: Caution:  For production, only allow particular origin
                  .allow_any_origin()
                  .send_wildcard()
                  .allowed_methods(vec!["GET"])
                  .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                  .allowed_header(http::header::CONTENT_TYPE)
                  .max_age(3600))
            .route("/wakeword/predict/{word}", web::get().to(handlers::predict_wake_word))
    })
    .bind(config.server_addr.clone())?
        .run()
        .await
}
