use tokio::sync::watch;
use tokio::{net::tcp::OwnedReadHalf, sync::mpsc};

use crate::parser::{Deserialize, Scanner};
use crate::protocol::packet::config::ServerboundPluginMessagePacket;
use crate::protocol::packet::handshake::HandshakePacket;
use crate::protocol::packet::login::LoginStartPacket;
use crate::protocol::packet::ping::PingRequestPacket;
use crate::protocol::{Packet, RawPacket};

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

            let packet: Option<Packet> = match state {
                ConnectionState::Handshake => handle_handshake_state(&raw_packet),
                ConnectionState::Status => handle_status_state(&raw_packet),
                ConnectionState::Login => handle_login_state(&raw_packet),
                ConnectionState::Configuration => handle_configuration_state(&raw_packet),
                _ => None,
            };

            if let Some(packet) = packet {
                tracing::info!("READ: {}", &packet);
                let _ = self.packet_sender.send(packet).await.map_err(|err| {
                    tracing::error!("{}", err);
                });
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

fn handle_handshake_state(raw_packet: &RawPacket) -> Option<Packet> {
    let mut scanner = Scanner::new(raw_packet.data());

    if raw_packet.packet_id() == 0x00 {
        return match HandshakePacket::deserialize(&mut scanner) {
            Ok(handshake_packet) => Some(Packet::Handshake(handshake_packet)),
            Err(_) => None,
        };
    }

    None
}

fn handle_status_state(raw_packet: &RawPacket) -> Option<Packet> {
    let mut scanner = Scanner::new(raw_packet.data());

    match raw_packet.packet_id() {
        0x00 => Some(Packet::StatusRequest),
        0x01 => PingRequestPacket::deserialize(&mut scanner)
            .map(|ping_request| Packet::PingRequest(ping_request))
            .ok(),
        _ => None,
    }
}

fn handle_login_state(raw_packet: &RawPacket) -> Option<Packet> {
    let mut scanner = Scanner::new(raw_packet.data());

    match raw_packet.packet_id() {
        0x00 => LoginStartPacket::deserialize(&mut scanner)
            .map(|login_start| Packet::LoginStart(login_start))
            .ok(),
        0x03 => Some(Packet::LoginAcknowledged),
        _ => None,
    }
}

fn handle_configuration_state(raw_packet: &RawPacket) -> Option<Packet> {
    let mut scanner = Scanner::new(raw_packet.data());

    match raw_packet.packet_id() {
        0x01 => ServerboundPluginMessagePacket::deserialize(&mut scanner)
            .map(|plugin_message| Packet::ServerboundPluginMessage(plugin_message))
            .ok(),
        0x02 => Some(Packet::AcknowledgeFinishConfiguration),
        _ => None,
    }
}
