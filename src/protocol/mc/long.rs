use bytes::{BufMut, BytesMut};

use crate::parser::{Deserialize, ParserResult, Scanner, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Long(i64);

impl Deserialize for Long {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let b1 = scanner.pop()?;
        let b2 = scanner.pop()?;
        let b3 = scanner.pop()?;
        let b4 = scanner.pop()?;
        let b5 = scanner.pop()?;
        let b6 = scanner.pop()?;
        let b7 = scanner.pop()?;
        let b8 = scanner.pop()?;

        let long = i64::from_be_bytes([b1, b2, b3, b4, b5, b6, b7, b8]);

        Ok(Self(long))
    }
}

impl Serialize for Long {
    fn serialize(&self, buffer: &mut BytesMut) {
        let bytes = self.0.to_be_bytes();
        buffer.put_slice(&bytes);
    }
}
