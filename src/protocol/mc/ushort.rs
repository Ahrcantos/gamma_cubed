use bytes::BytesMut;

use crate::parser::{Deserialize, ParserResult, Scanner, Serialize};

#[derive(Debug)]
pub struct UShort(pub u16);

impl Serialize for UShort {
    fn serialize(&self, buffer: &mut BytesMut) {
        todo!()
    }
}

impl Deserialize for UShort {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let high_byte = scanner.pop()?;
        let low_byte = scanner.pop()?;

        Ok(Self(u16::from_be_bytes([high_byte, low_byte])))
    }
}
