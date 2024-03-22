use bytes::{BufMut, Bytes, BytesMut};

use crate::parser::{byte_array, Deserialize, ParserResult, Scanner, Serialize};
use crate::protocol::mc::VarInt;

#[derive(Debug)]
pub struct RawPacket {
    packet_id: VarInt,
    length: VarInt,
    data: Bytes,
}

impl RawPacket {
    pub fn new(packet_id: VarInt, data: Bytes) -> Self {
        let length = VarInt::new(data.len() as i32);

        Self {
            packet_id,
            length,
            data,
        }
    }

    pub fn packet_id(&self) -> i32 {
        self.packet_id.inner()
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Deserialize for RawPacket {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let packet_length = VarInt::deserialize(scanner)?;

        let cursor_before = scanner.cursor();
        let packet_id = VarInt::deserialize(scanner)?;
        let cursor_after = scanner.cursor();

        let packet_id_length: i32 = (cursor_after - cursor_before) as i32;

        let data = byte_array(scanner, (packet_length.inner() - packet_id_length) as usize)?;

        Ok(Self {
            packet_id,
            length: packet_length,
            data,
        })
    }
}

impl Serialize for RawPacket {
    fn serialize(&self, mut buffer: &mut BytesMut) {
        self.length.serialize(&mut buffer);
        self.packet_id.serialize(&mut buffer);
        buffer.put_slice(&self.data);
    }
}
