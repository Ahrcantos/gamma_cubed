mod actors;
mod parser;
mod protocol;

use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing::Instrument;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, Registry, util::SubscriberInitExt, filter::LevelFilter};
use tracing_tree::HierarchicalLayer;

use crate::actors::ConnectionActorHandle;

#[tokio::main]
async fn main() {
    Registry::default().with(LevelFilter::TRACE).with(
        HierarchicalLayer::new(2)
            .with_targets(true)
            .with_bracketed_fields(true),
    ).init();

    let addr: SocketAddr = ([127, 0, 0, 1], 25565).into();
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server listening on address {}", &addr);

    while let Ok((stream, _)) = listener
        .accept()
        .instrument(tracing::info_span!("accept"))
        .await
    {
        let _ = ConnectionActorHandle::new(stream);
    }
}
