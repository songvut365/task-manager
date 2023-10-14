mod configuration;
mod database;
mod handler;
mod middleware;
mod model;
mod repository;
mod router;
mod service;
mod util;

use crate::configuration::config_yaml::load_config;
use crate::configuration::model::AppConfig;
use crate::router::task_manager::config_route;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use log::{error, info};
use std::sync::Mutex;
use std::{io, process};
// use tracing_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // normal log
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // json log
    // tracing_subscriber::fmt().json().init();

    let app_config: AppConfig = match load_config("config/config.yaml") {
        Ok(config) => {
            info!("config: {:?}", config);
            config
        }
        Err(err) => {
            error!("load config yaml error: {:?}", err);

            // exit by os error code
            if let Some(io_err) = err.downcast_ref::<io::Error>() {
                if let Some(err_code) = io_err.raw_os_error() {
                    process::exit(err_code)
                }
            }

            // default exit code is 1
            process::exit(1)
        }
    };

    // database
    let db_connection =
        database::postgres::connect_database(app_config.database).unwrap_or_else(|err| {
            error!("connect database error: {:?}", err);
            process::exit(1);
        });

    // redis
    let mut redis_client = database::cache::Client::new(app_config.redis);
    redis_client
        .connect_redis()
        .unwrap_or_else(|err| error!("connect redis error: {:}", err));

    // component
    let task_repository =
        repository::task_manager::TaskRepository::new(db_connection, redis_client);
    let task_service = service::task_manager::TaskService::new(task_repository);

    // inject service to handler
    let data_task_service = web::Data::new(Mutex::new(task_service));

    // start server
    info!(
        "Actix server is starting at {}:{}",
        app_config.http_server.address, app_config.http_server.port
    );
    HttpServer::new(move || {
        App::new()
            .configure(config_route)
            .wrap(middleware::logger::Logger {})
            .wrap(Logger::new(
                "timestamp: %t | method: %r | code: %s | latency: %D",
            ))
            .app_data(web::Data::clone(&data_task_service))
    })
    .workers(4)
    .bind((
        app_config.http_server.address.to_string(),
        app_config.http_server.port,
    ))?
    .run()
    .await?;

    info!("Actix server is shutting down...");
    Ok(())
}
