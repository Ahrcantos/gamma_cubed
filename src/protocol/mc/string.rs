use bytes::BufMut;

use crate::parser::{Deserialize, ParserError, ParserResult, Scanner, Serialize};

use super::VarInt;

#[derive(Debug)]
pub struct String(std::string::String);

impl String {
    pub fn new(input: &str) -> Self {
        Self(std::string::String::from(input))
    }
}

impl Serialize for String {
    fn serialize(&self, buffer: &mut bytes::BytesMut) {
        let bytes = self.0.as_bytes();
        let length = bytes.len();
        let length = VarInt::new(length as i32);

        length.serialize(buffer);
        buffer.put_slice(bytes);
    }
}

impl Deserialize for String {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let length = VarInt::deserialize(scanner)?;

        let data = scanner.pop_many(length.inner() as usize)?;

        match std::str::from_utf8(data) {
            Ok(i) => Ok(Self(std::string::String::from(i))),
            Err(_) => Err(ParserError::Unexpected),
        }
    }
}
