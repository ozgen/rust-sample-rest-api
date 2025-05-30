use hyper::{Response, StatusCode};
use http_body_util::Full;
use bytes::Bytes;

pub async fn hello_handler() -> Result<Response<Full<Bytes>>, hyper::Error> {
    let body = Full::new(Bytes::from_static(b"Hello, world!"));
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .unwrap())
}
