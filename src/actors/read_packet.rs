use tokio::sync::watch;
use tokio::{net::tcp::OwnedReadHalf, sync::mpsc};

use crate::parser::{Deserialize, Scanner};
use crate::protocol::packet::handshake::HandshakePacket;
use crate::protocol::packet::login::LoginStartPacket;
use crate::protocol::packet::ping::PingRequestPacket;
use crate::protocol::Packet;

use super::connection::ConnectionState;
use super::ReadRawPacketActorHandle;

struct ReadPacketActor {
    read_raw_packet_handle: ReadRawPacketActorHandle,
    packet_sender: mpsc::Sender<Packet>,
    connection_state_receiver: watch::Receiver<ConnectionState>,
}

impl ReadPacketActor {
    async fn run(mut self) {
        while let Some(raw_packet) = self.read_raw_packet_handle.recv().await {
            let state = *self.connection_state_receiver.borrow();

            if raw_packet.packet_id() == 0x00 && state == ConnectionState::Handshake {
                let mut scanner = Scanner::new(raw_packet.data());

                if let Ok(hsp) = HandshakePacket::deserialize(&mut scanner) {
                    self.packet_sender
                        .send(Packet::Handshake(hsp))
                        .await
                        .unwrap();
                }
            } else if raw_packet.packet_id() == 0x00 && state == ConnectionState::Status {
                self.packet_sender
                    .send(Packet::StatusRequest)
                    .await
                    .unwrap();
            } else if raw_packet.packet_id() == 0x00 && state == ConnectionState::Login {
                let mut scanner = Scanner::new(raw_packet.data());

                if let Ok(lsp) = LoginStartPacket::deserialize(&mut scanner) {
                    self.packet_sender
                        .send(Packet::LoginStart(lsp))
                        .await
                        .unwrap();
                }
            } else if raw_packet.packet_id() == 0x01 && state == ConnectionState::Status {
                let mut scanner = Scanner::new(raw_packet.data());

                if let Ok(prp) = PingRequestPacket::deserialize(&mut scanner) {
                    self.packet_sender
                        .send(Packet::PingRequest(prp))
                        .await
                        .unwrap();
                }
            } else {
                dbg!(raw_packet);
            }
        }
    }
}

impl ReadPacketActor {}

pub struct ReadPacketActorHandle {
    packet_receiver: mpsc::Receiver<Packet>,
}

impl ReadPacketActorHandle {
    pub fn new(
        reader: OwnedReadHalf,
        connection_state_receiver: watch::Receiver<ConnectionState>,
    ) -> Self {
        let (packet_sender, packet_receiver) = mpsc::channel::<Packet>(8);
        let read_raw_packet_handle = ReadRawPacketActorHandle::new(reader);

        let actor = ReadPacketActor {
            read_raw_packet_handle,
            packet_sender,
            connection_state_receiver,
        };

        tokio::spawn(actor.run());

        Self { packet_receiver }
    }

    pub async fn recv(&mut self) -> Option<Packet> {
        self.packet_receiver.recv().await
    }
}
