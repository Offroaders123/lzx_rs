use std::{fs, io::Cursor, path::PathBuf};

use byteorder::{BigEndian, ReadBytesExt};
use lzx_rs::x_decompress;

pub trait ConsoleParser {
    fn inflate_from_layout(&mut self, in_file_path: &PathBuf) -> Result<Vec<u8>, Status>;
    fn inflate_listing(&self) -> Result<Vec<u8>, Status>;
}

pub struct SaveLayout;
pub struct SaveProject;
pub struct WriteSettings;
pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn allocate(&mut self, n: usize) -> bool {
        self.data = vec![0; n];
        true
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

pub struct Xbox360Dat {
    m_file_path: Option<PathBuf>,
}

impl Xbox360Dat {
    pub fn new() -> Self {
        Xbox360Dat {
            m_file_path: None,
        }
    }
}

impl ConsoleParser for Xbox360Dat {
    fn inflate_from_layout(&mut self, the_file_path: &PathBuf) -> Result<Vec<u8>, Status> {
        self.m_file_path = Some(the_file_path.clone());

        self.inflate_listing()
    }

    fn inflate_listing(&self) -> Result<Vec<u8>, Status> {
        let file_path: &PathBuf = match &self.m_file_path {
            Some(path) => path,
            None => Err(Status::FileError)?,
        };

        let file_data: Buffer = match fs::read(file_path) {
            Ok(bytes) => {
                let mut buf: Buffer = Buffer::new();
                buf.data = bytes;
                buf
            }
            Err(_) => Err(Status::FileError)?,
        };

        if file_data.size() < 12 {
            return Err(Status::FileError);
        }
        let mut reader = Cursor::new(file_data.data());

        let src_size: u32 = match reader.read_u32::<BigEndian>() {
            Ok(val) => val.wrapping_sub(8),
            Err(_) => Err(Status::FileError)?,
        };

        let _skip: () = match reader.read_i32::<BigEndian>() {
            Ok(_) => (),
            Err(_) => Err(Status::FileError)?,
        };

        let file_size: u32 = match reader.read_u32::<BigEndian>() {
            Ok(val) => val,
            Err(_) => Err(Status::FileError)?,
        };

        // Allocate output buffer
        let mut inflated_data: Buffer = Buffer::new();
        if !inflated_data.allocate(file_size as usize) {
            return Err(Status::MallocFailed);
        }

        // Perform decompression
        let src_slice: &[u8] = &reader.into_inner()[8..(8 + src_size as usize)];
        let dst_slice: &mut [u8] = inflated_data.data_mut();

        let bytes: Vec<u8> = match x_decompress(src_slice, dst_slice) {
            Ok(_) => dst_slice.to_vec(),
            Err(err) => Err(Status::Decompress)?,
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
