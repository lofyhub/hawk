mod corruption_cases;
mod models;
mod politicians;
mod repository;
mod upload;
mod user_reviews;
mod utils;
mod votes;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{error, HttpResponse};
use actix_web::{http::header, web, App, HttpServer};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use env_logger::Env;
// use actix_web::FromRequest;

#[macro_use]
extern crate diesel_migrations;

use crate::corruption_cases::main::{
    get_corruption_cases, get_most_upvoted_cases, save_corruption_case,
};
use crate::politicians::main::{
    echo, get_corrupt_politician, get_corrupt_politicians, health, save_corrupt_politicians,
};
use crate::user_reviews::main::{get_case_review, save_case_review};
use crate::votes::main::{
    save_case_downvote, save_case_upvote, save_review_downvote, save_review_upvote,
};

type DB = diesel::pg::Pg;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations(connection: &mut impl MigrationHarness<DB>) {
    let _ = connection.run_pending_migrations(MIGRATIONS);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port_str = std::env::var("PORT").unwrap();
    let port: u16 = port_str
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let politicians_db = repository::database::Database::new();
    run_migrations(&mut politicians_db.pool.get().unwrap());
    let app_data = web::Data::new(politicians_db);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Allow bursts with up to five requests per IP address
    // and replenishes one element every two seconds
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        let json_cfg = web::JsonConfig::default()
            .limit(10 * 1024 * 1024) // 10 MB limit
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
            });

        let multipart_cfg = web::FormConfig::default().limit(10 * 1024 * 1024);
        let payload_cfg = web::PayloadConfig::default().limit(10 * 1024 * 1024);

        App::new()
            .app_data(app_data.clone())
            .app_data(json_cfg.clone())
            .app_data(multipart_cfg.clone())
            .app_data(payload_cfg.clone())
            .wrap(Governor::new(&governor_conf))
            .wrap(cors)
            .service(health)
            .service(echo)
            .service(get_corrupt_politician)
            .service(get_corrupt_politicians)
            .service(save_corrupt_politicians)
            .service(get_corruption_cases)
            .service(save_corruption_case)
            .service(get_case_review)
            .service(save_case_review)
            .service(save_case_downvote)
            .service(save_case_upvote)
            .service(save_review_downvote)
            .service(save_review_upvote)
            .service(get_most_upvoted_cases)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
