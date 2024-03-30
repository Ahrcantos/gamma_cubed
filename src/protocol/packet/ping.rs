use crate::{
    parser::{Deserialize, ParserResult, Scanner, Serialize},
    protocol::mc,
};

#[derive(Debug)]
pub struct PingRequestPacket {
    payload: mc::Long,
}

impl Deserialize for PingRequestPacket {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let payload = mc::Long::deserialize(scanner)?;

        Ok(Self { payload })
    }
}

#[derive(Debug)]
pub struct PingResponsePacket {
    payload: mc::Long,
}

impl From<PingRequestPacket> for PingResponsePacket {
    fn from(value: PingRequestPacket) -> Self {
        Self {
            payload: value.payload,
        }
    }
}

impl Serialize for PingResponsePacket {
    fn serialize(&self, buffer: &mut bytes::BytesMut) {
        self.payload.serialize(buffer);
    }
}
