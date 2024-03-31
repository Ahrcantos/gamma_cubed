use bytes::{BufMut, Bytes, BytesMut};

pub trait Serialize {
    fn serialize(&self, buffer: &mut BytesMut);
}

pub trait Deserialize
where
    Self: Sized,
{
    fn deserialize(scanner: &mut Scanner) -> ParserResult<Self>;
}

pub struct Scanner<'i> {
    cursor: usize,
    input: &'i [u8],
}

impl<'i> Scanner<'i> {
    pub fn new(input: &'i [u8]) -> Self {
        Self { cursor: 0, input }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<u8> {
        self.input.get(self.cursor).map(|b| *b)
    }

    pub fn remaining(&self) -> usize {
       self.input.len() - self.cursor
    }

    pub fn pop(&mut self) -> ParserResult<u8> {
        match self.input.get(self.cursor) {
            Some(v) => {
                self.cursor += 1;

                Ok(*v)
            }
            None => Err(ParserError::NotEnough),
        }
    }

    pub fn pop_many(&mut self, length: usize) -> ParserResult<&'i [u8]> {
        match self.input.get(self.cursor..(self.cursor + length)) {
            Some(a) => {
                self.cursor += length;

                Ok(a)
            }
            None => Err(ParserError::NotEnough),
        }
    }

    pub fn pop_remaining(&mut self) -> ParserResult<&'i [u8]> {
        self.pop_many(self.remaining())
    }
}

#[derive(Debug)]
pub enum ParserError {
    Unexpected,
    NotEnough,
}

pub type ParserResult<T> = Result<T, ParserError>;

pub fn boolean(scanner: &mut Scanner) -> ParserResult<bool> {
    let value = scanner.pop()?;

    match value {
        0x00 => Ok(false),
        0x01 => Ok(true),
        _ => Err(ParserError::Unexpected),
    }
}

pub fn byte_array(scanner: &mut Scanner, length: usize) -> ParserResult<Bytes> {
    let mut buffer = BytesMut::with_capacity(length);

    for _ in 0..length {
        let value = scanner.pop()?;
        buffer.put_u8(value);
    }

    Ok(buffer.freeze())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean() {
        let mut scanner = Scanner::new(&[0x01]);

        let res = boolean(&mut scanner).unwrap();

        assert_eq!(res, true);
    }

    #[test]
    fn test_byte_array() {
        let mut scanner = Scanner::new(&[0x01, 0x02, 0x03, 0x04]);

        let res = byte_array(&mut scanner, 3).unwrap();

        assert_eq!(&res[..], &[0x01, 0x02, 0x03])
    }
}
