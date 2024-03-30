use bytes::BufMut;

use crate::parser::{Deserialize, ParserResult, Scanner, Serialize};

#[derive(Debug)]
pub struct Uuid(uuid::Uuid);

impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl Serialize for Uuid {
    fn serialize(&self, buffer: &mut bytes::BytesMut) {
        let [a4, a3, a2, a1, b4, b3, b2, b1, c4, c3, c2, c1, d4, d3, d2, d1] = self.0.to_bytes_le();

        buffer.put_slice(&[a1, a2, a3, a4, b1, b2, b3, b4, c1, c2, c3, c4, d1, d2, d3, d4]);
    }
}

impl Deserialize for Uuid {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let hb1 = scanner.pop()?;
        let hb2 = scanner.pop()?;
        let hb3 = scanner.pop()?;
        let hb4 = scanner.pop()?;
        let hb5 = scanner.pop()?;
        let hb6 = scanner.pop()?;
        let hb7 = scanner.pop()?;
        let hb8 = scanner.pop()?;

        let lb1 = scanner.pop()?;
        let lb2 = scanner.pop()?;
        let lb3 = scanner.pop()?;
        let lb4 = scanner.pop()?;
        let lb5 = scanner.pop()?;
        let lb6 = scanner.pop()?;
        let lb7 = scanner.pop()?;
        let lb8 = scanner.pop()?;

        let uuid = uuid::Uuid::from_bytes([
            hb1, hb2, hb3, hb4, hb5, hb6, hb7, hb8, lb1, lb2, lb3, lb4, lb5, lb6, lb7, lb8,
        ]);

        Ok(Self(uuid))
    }
}
