use crate::parser::{Deserialize, ParserResult, Scanner, Serialize};

use super::VarInt;

#[derive(Debug)]
pub struct Array<T: Serialize + Deserialize> {
    items: Vec<T>,
}

impl<T: Serialize + Deserialize> Array<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T: Serialize + Deserialize> Serialize for Array<T> {
    fn serialize(&self, buffer: &mut bytes::BytesMut) {
        let length = self.items.len();
        let length = VarInt::new(length as i32);
        length.serialize(buffer);

        for item in self.items.iter() {
            item.serialize(buffer);
        }
    }
}

impl<T: Serialize + Deserialize> Deserialize for Array<T> {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let length = VarInt::deserialize(scanner)?;
        let length = length.inner();

        let mut items: Vec<T> = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let item = T::deserialize(scanner)?;
            items.push(item);
        }

        Ok(Self { items })
    }
}
