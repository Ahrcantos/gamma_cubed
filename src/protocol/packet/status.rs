use bytes::BytesMut;

use crate::parser::Serialize;
use crate::protocol::mc;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StatusResponsePacket {
    version: Version,
    players: Players,
    favicon: Option<String>,
    #[serde(rename = "enforcesSecureChat")]
    enforces_secure_chat: bool,
    #[serde(rename = "previewsChat")]
    previews_chat: bool,
}

impl Serialize for StatusResponsePacket {
    fn serialize(&self, buffer: &mut BytesMut) {
        let data = serde_json::to_string(self).unwrap();

        let data = mc::String::new(&data);

        data.serialize(buffer);
    }
}

impl Default for StatusResponsePacket {
    fn default() -> Self {
        Self {
            version: Version::default(),
            players: Players::default(),
            favicon: None,
            enforces_secure_chat: false,
            previews_chat: false,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Version {
    name: String,
    protocol: u32,
}

impl Default for Version {
    fn default() -> Self {
        Self {
            name: String::from("1.20.2"),
            protocol: 764,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Players {
    max: u32,
    online: u32,
    sample: Vec<SampleUser>,
}

impl Default for Players {
    fn default() -> Self {
        Self {
            max: 100,
            online: 0,
            sample: vec![SampleUser::default()],
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct SampleUser {
    name: String,
    id: String,
}

impl Default for SampleUser {
    fn default() -> Self {
        Self {
            name: String::from("Ahrcantos"),
            id: String::from("4566e69f-c907-48ee-8d71-d7ba5aa00d20"),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Description {
    text: String,
}

impl Default for Description {
    fn default() -> Self {
        Self {
            text: String::from("My custom server"),
        }
    }
}
