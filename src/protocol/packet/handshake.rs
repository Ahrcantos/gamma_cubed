use bytes::{BufMut, BytesMut};

use crate::parser::{Deserialize, ParserError, ParserResult, Scanner, Serialize};
use crate::protocol::mc;

#[derive(Debug)]
pub struct HandshakePacket {
    pub protocol_version: mc::VarInt,
    pub server_address: mc::String,
    pub server_port: mc::UShort,
    pub next_state: NextState,
}

impl Serialize for HandshakePacket {
    fn serialize(&self, buffer: &mut BytesMut) {
        self.protocol_version.serialize(buffer);
        self.server_address.serialize(buffer);
        self.server_port.serialize(buffer);
        self.next_state.serialize(buffer);
    }
}

impl Deserialize for HandshakePacket {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let protocol_version = mc::VarInt::deserialize(scanner)?;
        let server_address = mc::String::deserialize(scanner)?;
        let server_port = mc::UShort::deserialize(scanner)?;
        let next_state = NextState::deserialize(scanner)?;

        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}

#[derive(Debug)]
pub enum NextState {
    Status,
    Login,
}

impl Serialize for NextState {
    fn serialize(&self, buffer: &mut BytesMut) {
        let value: u8 = match self {
            Self::Status => 0x01,
            Self::Login => 0x02,
        };

        buffer.put_u8(value);
    }
}

impl Deserialize for NextState {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let value = scanner.pop()?;

        match value {
            0x01 => Ok(Self::Status),
            0x02 => Ok(Self::Login),
            _ => Err(ParserError::Unexpected),
        }
    }
}
