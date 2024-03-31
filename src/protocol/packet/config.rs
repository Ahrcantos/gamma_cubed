use bytes::{Bytes, BytesMut};

use crate::{
    parser::{Deserialize, ParserResult, Scanner, Serialize},
    protocol::mc,
};

#[derive(Debug)]
pub struct ServerboundPluginMessagePacket {
    identifier: mc::String,
    data: Bytes,
}

impl Serialize for ServerboundPluginMessagePacket {
    fn serialize(&self, buffer: &mut bytes::BytesMut) {
        todo!()
    }
}

impl Deserialize for ServerboundPluginMessagePacket {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let identifier = mc::String::deserialize(scanner)?;
        let data = scanner.pop_remaining()?;
        let data = BytesMut::from(data).freeze();

        Ok(Self { identifier, data })
    }
}
