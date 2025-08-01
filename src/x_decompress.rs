use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    error, fmt,
    io::{BufRead, Cursor, Read},
};

use crate::{Lzx, peekable_byte_reader::PeekableByteReader};

#[derive(Debug)]
pub enum XmemErr {
    Ok,
    Overflow,
    BadData,
    LzxInit,
    LzxRun,
    BufferTooSmall,
}

impl fmt::Display for XmemErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                XmemErr::Ok => "no error",
                XmemErr::Overflow => "input exhausted",
                XmemErr::BadData => "invalid block header",
                XmemErr::LzxInit => "cannot initialise LZX",
                XmemErr::LzxRun => "LZX decompression failed",
                XmemErr::BufferTooSmall => "output buffer too small",
            }
        )
    }
}

impl error::Error for XmemErr {}

pub fn x_decompress(input: &[u8], output: &mut [u8]) -> Result<usize, XmemErr> {
    const CHUNK_SIZE: usize = 0x8000;
    let mut reader: Cursor<&[u8]> = Cursor::new(input);
    let mut writer: Vec<u8> = Vec::with_capacity(output.len());

    let mut lzx: Lzx = Lzx::new(17).map_err(|_| XmemErr::LzxInit)?;

    let mut src: Vec<u8> = vec![0u8; CHUNK_SIZE * 2];
    let mut dst: Vec<u8> = vec![0u8; CHUNK_SIZE];

    loop {
        let mut dst_size: usize = CHUNK_SIZE;

        {
            let peek: &[u8] = reader.fill_buf().map_err(|_| XmemErr::Overflow)?;
            if peek.is_empty() {
                break;
            }
        }

        if reader
            .peek_u8()
            .map_err(|_| XmemErr::Overflow)?
            .ok_or(XmemErr::Overflow)?
            == 0xFF
        {
            reader.consume(1);
            if (reader.get_ref().len() - reader.position() as usize) < 2 {
                return Err(XmemErr::Overflow);
            }
            dst_size = reader
                .read_u16::<LittleEndian>()
                .map_err(|_| XmemErr::Overflow)? as usize;
        }

        if (reader.get_ref().len() - reader.position() as usize) < 2 {
            return Err(XmemErr::Overflow);
        }
        let src_size: usize = reader
            .read_u16::<LittleEndian>()
            .map_err(|_| XmemErr::Overflow)? as usize;

        if src_size == 0 || src_size > src.len() || dst_size == 0 || dst_size > dst.len() {
            return Err(XmemErr::BadData);
        }

        if (reader.get_ref().len() - reader.position() as usize) < src_size {
            return Err(XmemErr::Overflow);
        }

        reader
            .read_exact(&mut src[..src_size])
            .map_err(|_| XmemErr::Overflow)?;

        lzx.decompress(&mut src[..src_size], &mut dst[..dst_size])
            .map_err(|_| XmemErr::LzxRun)?;

        writer.extend_from_slice(&dst[..dst_size]);

        if reader
            .peek_u8()
            .map_err(|_| XmemErr::Overflow)?
            .ok_or(XmemErr::Overflow)?
            == 0xFF
        {
            break;
        }
    }

    if writer.len() > output.len() {
        return Err(XmemErr::BufferTooSmall);
    }

    output[..writer.len()].copy_from_slice(&writer);
    Ok(writer.len())
}
