pub mod disconnect;
pub mod encryption;
pub mod handshake;
pub mod login;
pub mod status;
pub mod ping;

use crate::parser::Serialize;

use self::disconnect::DisconnectPacket;
use self::encryption::EncryptionRequestPacket;
use self::handshake::HandshakePacket;
use self::login::LoginStartPacket;
use self::status::StatusResponsePacket;
use self::ping::{PingRequestPacket, PingResponsePacket};

#[derive(Debug)]
pub enum Packet {
    Handshake(HandshakePacket),
    StatusRequest,
    StatusResponse(StatusResponsePacket),
    PingRequest(PingRequestPacket),
    PingResponse(PingResponsePacket),
    LoginStart(LoginStartPacket),
    Disconnect(DisconnectPacket),
    EncryptionRequest(EncryptionRequestPacket),
}

impl Packet {
    pub fn packet_id(&self) -> i32 {
        match self {
            Self::Handshake(_) => 0,
            Self::StatusRequest => 0,
            Self::StatusResponse(_) => 0,
            Self::PingRequest(_) => 1,
            Self::PingResponse(_) => 1,
            Self::LoginStart(_) => 0,
            Self::Disconnect(_) => 0,
            Self::EncryptionRequest(_) => 1,
        }
    }
}

impl Serialize for Packet {
    fn serialize(&self, buffer: &mut bytes::BytesMut) {
        match self {
            Self::Handshake(packet) => packet.serialize(buffer),
            Self::StatusRequest => {}
            Self::StatusResponse(packet) => packet.serialize(buffer),
            Self::PingRequest(_) => {},
            Self::PingResponse(packet) => packet.serialize(buffer),
            Self::LoginStart(_) => {}
            Self::Disconnect(packet) => packet.serialize(buffer),
            Self::EncryptionRequest(_) => {
                todo!()
            }
        }
    }
}
