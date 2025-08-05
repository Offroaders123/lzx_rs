use std::{fs, io::Cursor, path::PathBuf};

use byteorder::{BigEndian, ReadBytesExt};
use lzx_rs::x_decompress;

pub trait ConsoleParser {
    fn inflate_from_layout(&mut self, file_path: &PathBuf) -> Result<Vec<u8>, Status>;
    fn inflate_listing(&self) -> Result<Vec<u8>, Status>;
}

pub struct Xbox360Dat {
    file_path: Option<PathBuf>,
}

impl ConsoleParser for Xbox360Dat {
    fn inflate_from_layout(&mut self, file_path: &PathBuf) -> Result<Vec<u8>, Status> {
        self.file_path = Some(file_path.clone());

        self.inflate_listing()
    }

    fn inflate_listing(&self) -> Result<Vec<u8>, Status> {
        let file_path: &PathBuf = match &self.file_path {
            Some(path) => path,
            None => Err(Status::FileError)?,
        };

        let file_data: Vec<u8> = fs::read(file_path).map_err(|_| Status::FileError)?;

        if file_data.len() < 12 {
            return Err(Status::FileError);
        }
        let mut reader: Cursor<Vec<u8>> = Cursor::new(file_data);

        let src_size: u32 = reader
            .read_u32::<BigEndian>()
            .map_err(|_| Status::FileError)?
            .wrapping_sub(8);

        let _skip: i32 = reader
            .read_i32::<BigEndian>()
            .map_err(|_| Status::FileError)?;

        let file_size: u32 = reader
            .read_u32::<BigEndian>()
            .map_err(|_| Status::FileError)?;

        // Allocate output buffer
        let mut inflated_data: Vec<u8> = vec![0; file_size as usize];

        // Perform decompression
        let src_slice: &[u8] = &reader.into_inner()[8..(8 + src_size as usize)];
        let dst_slice: &mut [u8] = &mut inflated_data;

        let bytes: Vec<u8> =
            match x_decompress(src_slice, dst_slice).map_err(|_| Status::Decompress)? {
                _ => dst_slice.to_vec(),
            };

        if inflated_data.is_empty() {
            return Err(Status::Decompress);
        }

        Ok(bytes)
    }
}

pub enum Status {
    Compress = -1,
    Decompress = -2,
    MallocFailed = -3,
    InvalidSave = -4,
    FileError = -5,
    InvalidConsole = -6,
    InvalidArgument = -7,
    NotImplemented = -8,
}
