use std::{fs, io::Cursor, path::PathBuf};

use byteorder::{BigEndian, ReadBytesExt};
use lzx_rs::x_decompress;

pub trait ConsoleParser {
    // fn discover_save_layout(&self, root_folder: &PathBuf) -> SaveLayout;
    fn inflate_from_layout(&mut self, the_save: &SaveProject, in_file_path: &PathBuf) -> Result<&[u8], Status>;

    // fn deflate_to_save(&self, save_project: &SaveProject, the_settings: &WriteSettings) -> i32;
    // fn supply_required_defaults(&self, save_project: &SaveProject) -> ();

    // protected:

    fn inflate_listing(&self, save_project: &SaveProject) -> Result<&[u8], Status>;
    // fn deflate_listing(
    //     &self,
    //     game_data_path: &PathBuf,
    //     inflated_data: &Buffer,
    //     deflated_data: &Buffer,
    // ) -> i32;

    // fn read_file_info(&self, save_project: &SaveProject) -> ();
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

    pub fn with_capacity(n: usize) -> Self {
        Self {
            data: Vec::with_capacity(n),
        }
    }

    pub fn allocate(&mut self, n: usize) -> bool {
        self.data = vec![0; n];
        true
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn size_mut(&mut self) -> &mut usize {
        // Not idiomatic — but you can implement this way if needed
        let len = self.data.len();
        // Create a dummy mutable reference (rarely needed in idiomatic code)
        // In practice, redesign to avoid this pattern
        panic!("Avoid using size_mut like C++; restructure your logic.")
    }

    // pub fn size_ref(&self) -> &usize {
    //     &self.data.len()
    // }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn span(&self) -> &[u8] {
        &self.data
    }

    pub fn span_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.data
    }
}

pub struct Xbox360Dat {
    m_console: Console,
    m_file_path: Option<PathBuf>,
}

pub enum Console {
    Xbox360,
}

impl Xbox360Dat {
    pub fn new() -> Self {
        Xbox360Dat {
            m_console: Console::Xbox360,
            m_file_path: None,
        }
    }
}

impl ConsoleParser for Xbox360Dat {
    fn inflate_from_layout(
        &mut self,
        save_project: &SaveProject,
        the_file_path: &PathBuf,
    ) -> Result<&[u8], Status> {
        self.m_file_path = Some(the_file_path.clone());

        let status: Result<&[u8], Status> = self.inflate_listing(save_project);
        let bytes: &[u8] = match status {
            Ok(bytes) => bytes,
            _ => {
                println!("failed to extract listing\n");
                return status;
            }
        };

        // readFileInfo(save_project);

        return Ok(bytes);
    }

    fn inflate_listing(&self, save_project: &SaveProject) -> Result<&[u8], Status> {
        let file_path: &PathBuf = match &self.m_file_path {
            Some(path) => path,
            None => {
                eprintln!("ERROR_4: File path not set");
                return Err(Status::FileError);
            }
        };

        let file_data: Buffer = match fs::read(file_path) {
            Ok(bytes) => {
                let mut buf: Buffer = Buffer::new();
                buf.data = bytes;
                buf
            }
            Err(_) => {
                eprintln!("ERROR_4: {}", file_path.display());
                return Err(Status::FileError);
            }
        };

        // if (!saveProject.m_stateSettings.shouldDecompress()) {
        //     int status = FileListing::readListing(saveProject, fileData, m_console);
        //     return status;
        // }

        if file_data.size() < 12 {
            eprintln!("ERROR_5");
            return Err(Status::FileError);
        }
        let mut reader = Cursor::new(file_data.data());

        let src_size: u32 = match reader.read_u32::<BigEndian>() {
            Ok(val) => val.wrapping_sub(8),
            Err(_) => return Err(Status::FileError),
        };

        let _skip: () = match reader.read_i32::<BigEndian>() {
            Ok(_) => (),
            Err(_) => return Err(Status::FileError),
        };

        let file_size: u32 = match reader.read_u32::<BigEndian>() {
            Ok(val) => val,
            Err(_) => return Err(Status::FileError),
        };

        // Allocate output buffer
        let mut inflated_data: Buffer = Buffer::new();
        if !inflated_data.allocate(file_size as usize) {
            eprintln!("ERROR_1: {}", file_size);
            return Err(Status::MallocFailed);
        }

        // Perform decompression
        let src_slice: &[u8] = &reader.into_inner()[8..(8 + src_size as usize)];
        let dst_slice: &mut [u8] = inflated_data.data_mut();

        let bytes: &[u8] = match x_decompress(src_slice, dst_slice) {
            Ok(_) => dst_slice,
            Err(err) => {
                eprintln!("ERROR_3: ERROR_3 ({:?})", err);
                return Err(Status::Decompress);
            }
        };

        if inflated_data.is_empty() {
            eprintln!("ERROR_3");
            return Err(Status::Decompress);
        }

        Ok(bytes)
        // FileListing::read_listing(save_project, &inflated_data, &self.m_console)
    }
}

pub enum Status {
    // Success = 0,
    Compress = -1,
    Decompress = -2,
    MallocFailed = -3,
    InvalidSave = -4,
    FileError = -5,
    InvalidConsole = -6,
    InvalidArgument = -7,
    NotImplemented = -8,
}
