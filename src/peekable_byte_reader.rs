use std::io::{BufRead, Result};

pub trait PeekableByteReader {
    fn peek_u8(&mut self) -> Result<Option<u8>>;
}

impl<T: BufRead> PeekableByteReader for T {
    fn peek_u8(&mut self) -> Result<Option<u8>> {
        let buf: &[u8] = self.fill_buf()?;
        Ok(buf.first().copied())
    }
}
