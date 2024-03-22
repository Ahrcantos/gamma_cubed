use crate::{
    parser::{Deserialize, ParserResult, Scanner, Serialize},
    protocol::mc,
};

#[derive(Debug)]
pub struct LoginStartPacket {
    name: mc::String,
    player_id: mc::Uuid,
}

impl Serialize for LoginStartPacket {
    fn serialize(&self, buffer: &mut bytes::BytesMut) {
        todo!()
    }
}

impl Deserialize for LoginStartPacket {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let name = mc::String::deserialize(scanner)?;
        let player_id = mc::Uuid::deserialize(scanner)?;

        Ok(Self { name, player_id })
    }
}
