mod handlers;

use std::io;

use actix_web::{
    middleware,
    App, HttpServer,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Compress::default())
            .configure(handlers::init)
    })
    .bind_auto_h2c("0.0.0.0:80")?
    .run()
    .await?;

    Ok(())
}
