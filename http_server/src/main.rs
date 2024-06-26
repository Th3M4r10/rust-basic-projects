use hyper::{Request, Response, Server};
use hyper::body::Body;
use hyper::service::{make_service_fn, service_fn};
use hyper::http::StatusCode;
use std::convert::Infallible;


async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Hello, World"))
        .unwrap())
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::http(addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
