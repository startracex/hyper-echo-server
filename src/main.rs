mod services;
mod utils;

use std::error::Error;
use std::net::SocketAddr;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

async fn wrap_main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = std::env::args()
        .nth(1)
        .expect("Require port argument")
        .parse::<u16>()
        .expect("Fail parse number");

    println!("Listening and serve at http://localhost:{}", port);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(services::echo))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(wrap_main())
}
