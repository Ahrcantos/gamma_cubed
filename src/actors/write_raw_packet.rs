use bytes::BytesMut;
use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf, sync::mpsc};

use crate::parser::Serialize;
use crate::protocol::RawPacket;

struct WriteRawPacketActor {
    receiver: mpsc::Receiver<RawPacket>,
    writer: OwnedWriteHalf,
}

impl WriteRawPacketActor {
    async fn handle_message(&mut self, packet: RawPacket) {
        let mut buffer = BytesMut::new();
        packet.serialize(&mut buffer);

        self.writer.write(&buffer).await.unwrap();
    }

    async fn run(mut self) {
        while let Some(packet) = self.receiver.recv().await {
            self.handle_message(packet).await;
        }
    }
}

#[derive(Clone)]
pub struct WriteRawPacketActorHandle {
    sender: mpsc::Sender<RawPacket>,
}

impl WriteRawPacketActorHandle {
    pub fn new(writer: OwnedWriteHalf) -> Self {
        let (sender, receiver) = mpsc::channel::<RawPacket>(8);
        let actor = WriteRawPacketActor { receiver, writer };
        tokio::spawn(actor.run());

        Self { sender }
    }

    pub async fn send(&self, packet: RawPacket) {
        self.sender.send(packet).await.unwrap();
    }
}
