mod actors;
mod parser;
mod protocol;

use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::actors::ConnectionActorHandle;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = ([127, 0, 0, 1], 25565).into();
    let listener = TcpListener::bind(addr).await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let _ = ConnectionActorHandle::new(stream);
    }
}
