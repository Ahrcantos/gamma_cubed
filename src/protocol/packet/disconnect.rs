use bytes::BytesMut;

use crate::{parser::Serialize, protocol::mc};

#[derive(Debug)]
pub struct DisconnectPacket {
    reason: String,
}

impl DisconnectPacket {
    pub fn reason(reason: &str) -> Self {
        Self {
            reason: String::from(reason),
        }
    }
}

impl Serialize for DisconnectPacket {
    fn serialize(&self, buffer: &mut BytesMut) {
        #[derive(serde::Serialize)]
        struct JsonText<'i> {
            text: &'i str,
        }

        let text = JsonText { text: &self.reason };
        let data = serde_json::to_string(&text).unwrap();
        let data = mc::String::new(&data);
        data.serialize(buffer);
    }
}
