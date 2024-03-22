use bytes::{BufMut, BytesMut};
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf, sync::mpsc};

use crate::parser::{Deserialize, ParserError, Scanner};
use crate::protocol::RawPacket;

struct ReadRawPacketActor {
    packet_sender: mpsc::Sender<RawPacket>,
    reader: OwnedReadHalf,
    buffer: BytesMut,
}

impl ReadRawPacketActor {
    async fn run(mut self) {
        let mut buffer_tmp = [0u8; 1024];

        while let Ok(read_bytes) = self.reader.read(&mut buffer_tmp).await {
            self.buffer.put_slice(&buffer_tmp[0..read_bytes]);

            let mut scanner = Scanner::new(&self.buffer);
            let packet = RawPacket::deserialize(&mut scanner);

            match packet {
                Ok(packet) => {
                    let pos = scanner.cursor();
                    let _ = self.buffer.split_to(pos);
                    self.packet_sender.send(packet).await.unwrap();
                }
                Err(ParserError::NotEnough) => {
                    break;
                }
                Err(ParserError::Unexpected) => {
                    return;
                }
            }
        }
    }
}

pub struct ReadRawPacketActorHandle {
    packet_receiver: mpsc::Receiver<RawPacket>,
}

impl ReadRawPacketActorHandle {
    pub fn new(reader: OwnedReadHalf) -> Self {
        let (packet_sender, packet_receiver) = mpsc::channel::<RawPacket>(8);
        let actor = ReadRawPacketActor {
            packet_sender,
            reader,
            buffer: BytesMut::with_capacity(1024),
        };
        tokio::spawn(actor.run());

        Self { packet_receiver }
    }

    pub async fn recv(&mut self) -> Option<RawPacket> {
        self.packet_receiver.recv().await
    }
}
