mod config;
mod handlers;
mod routes;

use crate::config::Config;
use crate::routes::router;

use dotenvy::dotenv;
use sqlx::PgPool;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = Config::from_env()?;
    let db_pool = PgPool::connect(&config.database_url).await?;

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on http://127.0.0.1:8080");

    loop {
        let (stream, _) = listener.accept().await?;
        let db_pool = db_pool.clone();

        tokio::spawn(async move {
            let io = TokioIo::new(stream);

            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| router(req, db_pool.clone())))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
