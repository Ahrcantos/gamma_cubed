pub mod config;
pub mod disconnect;
pub mod encryption;
pub mod handshake;
pub mod login;
pub mod ping;
pub mod status;

use std::fmt::Display;

use crate::parser::Serialize;

use self::config::ServerboundPluginMessagePacket;
use self::disconnect::DisconnectPacket;
use self::encryption::EncryptionRequestPacket;
use self::handshake::HandshakePacket;
use self::login::{LoginStartPacket, LoginSuccessPacket};
use self::ping::{PingRequestPacket, PingResponsePacket};
use self::status::StatusResponsePacket;

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
    LoginSuccess(LoginSuccessPacket),
    LoginAcknowledged,
    // Config
    ServerboundPluginMessage(ServerboundPluginMessagePacket),
    FinishConfiguration,
    AcknowledgeFinishConfiguration,
}

impl Packet {
    pub fn packet_id(&self) -> i32 {
        match self {
            Self::Handshake(_) => 0x00,
            Self::StatusRequest => 0x00,
            Self::StatusResponse(_) => 0x00,
            Self::PingRequest(_) => 0x01,
            Self::PingResponse(_) => 0x01,
            Self::LoginStart(_) => 0x00,
            Self::Disconnect(_) => 0x00,
            Self::EncryptionRequest(_) => 0x01,
            Self::LoginSuccess(_) => 0x02,
            Self::LoginAcknowledged => 0x03,
            // Config
            Self::ServerboundPluginMessage(_) => 0x01,
            Self::FinishConfiguration => 0x02,
            Self::AcknowledgeFinishConfiguration => 0x02,
        }
    }
}

impl Serialize for Packet {
    fn serialize(&self, buffer: &mut bytes::BytesMut) {
        match self {
            Self::Handshake(packet) => packet.serialize(buffer),
            Self::StatusRequest => {}
            Self::StatusResponse(packet) => packet.serialize(buffer),
            Self::PingRequest(_) => {}
            Self::PingResponse(packet) => packet.serialize(buffer),
            Self::LoginStart(_) => {}
            Self::Disconnect(packet) => packet.serialize(buffer),
            Self::EncryptionRequest(_) => {
                todo!()
            }
            Self::LoginSuccess(packet) => packet.serialize(buffer),
            Self::LoginAcknowledged => {}
            // Config
            Self::ServerboundPluginMessage(_) => {}
            Self::FinishConfiguration => {}
            Self::AcknowledgeFinishConfiguration => {}
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Handshake(_) => write!(f, "Handshake"),
            Self::StatusRequest => write!(f, "StatusRequest"),
            Self::StatusResponse(_) => write!(f, "StatusResponse"),
            Self::PingRequest(_) => write!(f, "PingRequest"),
            Self::PingResponse(_) => write!(f, "PingResponse"),
            Self::LoginStart(_) => write!(f, "LoginStart"),
            Self::Disconnect(_) => write!(f, "Disconnect"),
            Self::EncryptionRequest(_) => write!(f, "EncryptionRequest"),
            Self::LoginSuccess(_) => write!(f, "LoginSuccess"),
            Self::LoginAcknowledged => write!(f, "LoginAcknowledged"),
            Self::ServerboundPluginMessage(_) => write!(f, "ServerboundPluginMessage"),
            Self::FinishConfiguration => write!(f, "FinishConfiguration"),
            Self::AcknowledgeFinishConfiguration => write!(f, "AcknowledgeFinishConfiguration"),
        }
    }
}