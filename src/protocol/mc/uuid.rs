use crate::parser::{Deserialize, ParserResult, Scanner, Serialize};

#[derive(Debug)]
pub struct Uuid(uuid::Uuid);

impl Serialize for Uuid {
    fn serialize(&self, buffer: &mut bytes::BytesMut) {
        todo!()
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
