use crate::protocol::mc;

#[derive(Debug)]
pub struct EncryptionRequestPacket {
    server_id: mc::String,
    public_key: mc::ByteArray,
    verify_token: mc::ByteArray,
}
