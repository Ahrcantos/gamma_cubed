use bytes::{BufMut, Bytes, BytesMut};

use crate::{parser::Serialize, protocol::mc::VarInt};

#[derive(Debug)]
pub struct ByteArray(Bytes);

impl ByteArray {
    pub fn new(bytes: Bytes) -> Self {
        Self(bytes)
    }
}

impl Serialize for ByteArray {
    fn serialize(&self, buffer: &mut BytesMut) {
        let length = self.0.len();
        let length = VarInt::new(length as i32);

        length.serialize(buffer);
        buffer.put_slice(&self.0);
    }
}
