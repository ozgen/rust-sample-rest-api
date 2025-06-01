use hyper::{Request, Response, Method, StatusCode};
use hyper::body::Incoming;
use sqlx::PgPool;
use bytes::Bytes;
use http_body_util::{Full, BodyExt};

use crate::handlers::hello_handler;

pub async fn router(
    req: Request<Incoming>, // <- Incoming replaces Body
    _pool: PgPool,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => hello_handler().await,
        _ => {
            let body = Full::new(Bytes::from_static(b"Not Found"));
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body)
                .unwrap())
        }
    }
}
