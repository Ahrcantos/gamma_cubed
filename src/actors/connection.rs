use tokio::{net::TcpStream, sync::watch};

use crate::protocol::{
    packet::{
        handshake::{HandshakePacket, NextState},
        login::LoginSuccessPacket,
        ping::PingResponsePacket,
        status::StatusResponsePacket,
    },
    Packet,
};

use super::{ReadPacketActorHandle, WritePacketActorHandle};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Configuration,
}

struct ConnectionActor {
    connection_state_sender: watch::Sender<ConnectionState>,
    read_packet_handle: ReadPacketActorHandle,
    write_packet_handle: WritePacketActorHandle,
}

impl ConnectionActor {
    async fn run(mut self) {
        while let Some(incoming_packet) = self.read_packet_handle.recv().await {
            if let Packet::Handshake(HandshakePacket { next_state, .. }) = incoming_packet {
                let state = match next_state {
                    NextState::Status => ConnectionState::Status,
                    NextState::Login => ConnectionState::Login,
                };

                let _ = self.connection_state_sender.send(state);
                continue;
            }

            if let Packet::StatusRequest = incoming_packet {
                let _ = self
                    .write_packet_handle
                    .send(Packet::StatusResponse(StatusResponsePacket::default()))
                    .await;
                continue;
            }

            if let Packet::PingRequest(packet) = incoming_packet {
                let _ = self
                    .write_packet_handle
                    .send(Packet::PingResponse(PingResponsePacket::from(packet)))
                    .await;
                continue;
            }

            if let Packet::LoginStart(_) = incoming_packet {
                let _ = self
                    .write_packet_handle
                    .send(Packet::LoginSuccess(LoginSuccessPacket::default()))
                    .await;
                continue;
            }

            if let Packet::LoginAcknowledged = incoming_packet {
                let _ = self
                    .connection_state_sender
                    .send(ConnectionState::Configuration);

                let _ = self
                    .write_packet_handle
                    .send(Packet::FinishConfiguration)
                    .await;
            }
        }
    }
}

pub struct ConnectionActorHandle {
    connection_state_receiver: watch::Receiver<ConnectionState>,
}

impl ConnectionActorHandle {
    pub fn new(stream: TcpStream) -> Self {
        let (read_half, write_half) = stream.into_split();

        let (connection_state_sender, connection_state_receiver) =
            watch::channel::<ConnectionState>(ConnectionState::Handshake);

        let read_packet_handle =
            ReadPacketActorHandle::new(read_half, connection_state_receiver.clone());
        let write_packet_handle = WritePacketActorHandle::new(write_half);

        let actor = ConnectionActor {
            connection_state_sender,
            read_packet_handle,
            write_packet_handle,
        };

        tokio::spawn(actor.run());

        Self {
            connection_state_receiver,
        }
    }
}
