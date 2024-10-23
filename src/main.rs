mod forms;
mod handlers;
mod services;

use std::{env, io, sync::Arc};

use actix_web::{middleware, web, App, HttpServer};
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DbConn};
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    db: DbConn,
    online_users: Arc<Mutex<u64>>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let db = Database::connect(&db_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let state = AppState {
        db,
        online_users: Arc::new(Mutex::new(0)),
    };

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Compress::default())
            .configure(handlers::init)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };

    log::info!("Starting server at {server_url}");
    server.run().await?;

    Ok(())
}
