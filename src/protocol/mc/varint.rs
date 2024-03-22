use bytes::BufMut;

use crate::parser::{Deserialize, ParserError, ParserResult, Scanner, Serialize};

#[derive(Debug)]
pub struct VarInt(i32);

impl VarInt {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub fn inner(&self) -> i32 {
        self.0
    }
}

impl Serialize for VarInt {
    fn serialize(&self, buffer: &mut bytes::BytesMut) {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut value = self.0 as u32;
        loop {
            if value & !(SEGMENT_BITS as u32) == 0x00 {
                buffer.put_u8(value as u8);
                break;
            }

            buffer.put_u8(((value & SEGMENT_BITS as u32) | CONTINUE_BIT as u32) as u8);
            value = value >> 7;
        }
    }
}

impl Deserialize for VarInt {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut value: u32 = 0;
        let mut position = 0;

        loop {
            let current_byte = scanner.pop()?;

            value = value | (((current_byte as u32) & SEGMENT_BITS as u32) << position);

            if current_byte & CONTINUE_BIT == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                return Err(ParserError::Unexpected);
            }
        }

        Ok(VarInt::new(value as i32))
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn test_varint_serialize() {
        let value = VarInt::new(4609);
        let mut buffer = BytesMut::new();
        value.serialize(&mut buffer);

        assert_eq!(&[0x81, 0x24], &buffer[..]);
    }

    #[test]
    fn test_varint_deserialize() {
        let input = &[0x81, 0x24];
        let mut scanner = Scanner::new(input);

        let res = VarInt::deserialize(&mut scanner).unwrap();

        assert_eq!(4609, res.inner());

        let input = &[0xfc, 0x05];
        let mut scanner = Scanner::new(input);

        let res = VarInt::deserialize(&mut scanner).unwrap();

        assert_eq!(764, res.inner());
    }
}
