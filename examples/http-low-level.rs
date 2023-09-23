use futures_util::stream::StreamExt;
///! This uses the lower-level Http struct rather than the Server struct,
///! which is likely the direction hyper will go in.
///! One issue with this approach is it is more difficult to handle graceful shutdown.
///! See https://github.com/hyperium/hyper/issues/2321
use hyper::server::conn::{AddrIncoming, Http};
use hyper::service::service_fn;
use hyper::{Body, Response};
use std::convert::Infallible;

mod tls_config;
use tls_config::tls_acceptor;

/// For example try:
/// `curl https://localhost:3000 -k`
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let listener = tls_listener::builder(tls_acceptor())
        .max_handshakes(10)
        .listen(AddrIncoming::bind(&addr).unwrap());

    let svc =
        service_fn(|_| async { Ok::<_, Infallible>(Response::new(Body::from("Hello, World!"))) });

    let http = Http::new();
    listener
        .for_each(|r| async {
            match r {
                Ok((conn, remote_addr)) => {
                    let http = http.clone();
                    tokio::spawn(async move {
                        if let Err(err) = http.serve_connection(conn, svc).await {
                            eprintln!("[client {remote_addr}] Application error: {}", err);
                        }
                    });
                }
                Err(err) => {
                    if let Some(remote_addr) = err.remote_addr() {
                        eprint!("[client {remote_addr}] ");
                    }

                    eprintln!("Error accepting connection: {}", err);
                }
            }
        })
        .await;
}
