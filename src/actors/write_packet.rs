use bytes::BytesMut;
use tokio::{net::tcp::OwnedWriteHalf, sync::mpsc};

use crate::{
    parser::Serialize,
    protocol::{mc::VarInt, Packet, RawPacket},
};

use super::WriteRawPacketActorHandle;

struct WritePacketActor {
    packet_receiver: mpsc::Receiver<Packet>,
    write_raw_packet_handle: WriteRawPacketActorHandle,
}

impl WritePacketActor {
    async fn run(mut self) {
        while let Some(packet) = self.packet_receiver.recv().await {
            let mut buffer = BytesMut::new();
            packet.serialize(&mut buffer);
            let packet_id = VarInt::new(packet.packet_id());
            let raw_packet = RawPacket::new(packet_id, buffer.freeze());

            self.write_raw_packet_handle.send(raw_packet).await;
        }
    }
}

pub struct WritePacketActorHandle {
    packet_sender: mpsc::Sender<Packet>,
}

impl WritePacketActorHandle {
    pub fn new(writer: OwnedWriteHalf) -> Self {
        let (packet_sender, packet_receiver) = mpsc::channel::<Packet>(8);

        let write_raw_packet_handle = WriteRawPacketActorHandle::new(writer);

        let actor = WritePacketActor {
            write_raw_packet_handle,
            packet_receiver,
        };

        tokio::spawn(actor.run());

        Self { packet_sender }
    }

    pub async fn send(&mut self, packet: Packet) {
        let _ = self.packet_sender.send(packet).await;
    }
}
