use bytes::{BufMut, BytesMut};

use crate::{
    parser::{Deserialize, ParserError, ParserResult, Scanner, Serialize},
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

#[derive(Debug)]
pub struct LoginSuccessPacket {
    uuid: mc::Uuid,
    username: mc::String,
    properties: mc::Array<Property>,
}

impl Default for LoginSuccessPacket {
    fn default() -> Self {

        let id = uuid::uuid!("44f348cb-84b0-4fb9-a5c0-8eeb4596d6ba");
        Self {
            uuid: mc::Uuid::from(id),
            username: mc::String::from("Ahrcantos"),
            properties: mc::Array::new(),
        }
    }
}

impl Serialize for LoginSuccessPacket {
    fn serialize(&self, buffer: &mut BytesMut) {
        self.uuid.serialize(buffer);
        self.username.serialize(buffer);
        self.properties.serialize(buffer);
    }
}

impl Deserialize for LoginSuccessPacket {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let uuid = mc::Uuid::deserialize(scanner)?;
        let username = mc::String::deserialize(scanner)?;

        let properties = mc::Array::<Property>::deserialize(scanner)?;

        Ok(Self {
            uuid,
            username,
            properties,
        })
    }
}


#[derive(Debug)]
struct Property {
    name: mc::String,
    value: mc::String,
    signature: Option<mc::String>,
}

impl Serialize for Property {
    fn serialize(&self, buffer: &mut BytesMut) {
        self.name.serialize(buffer);
        self.value.serialize(buffer);

        match &self.signature {
            Some(signature) => {
                buffer.put_u8(0x01);
                signature.serialize(buffer);
            }
            None => {
                buffer.put_u8(0x00);
            }
        }
    }
}

impl Deserialize for Property {
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self> {
        let name = mc::String::deserialize(scanner)?;
        let value = mc::String::deserialize(scanner)?;

        let has_signature_byte = scanner.pop()?;

        match has_signature_byte {
            0x00 => Ok(Self {
                name,
                value,
                signature: None,
            }),
            0x01 => {
                let signature = mc::String::deserialize(scanner)?;

                Ok(Self {
                    name,
                    value,
                    signature: Some(signature),
                })
            }

            _ => Err(ParserError::Unexpected),
        }
    }
}