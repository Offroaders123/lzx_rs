use std::{fs::read, io};

use lzx_rs::{XmemErr, x_decompress};

#[derive(Debug)]
enum DecompressError {
    Io(io::Error),
    XmemErr(XmemErr),
}

impl From<io::Error> for DecompressError {
    fn from(e: io::Error) -> Self {
        DecompressError::Io(e)
    }
}

impl From<XmemErr> for DecompressError {
    fn from(e: XmemErr) -> Self {
        DecompressError::XmemErr(e)
    }
}

#[test]
fn decompress_xbox_360() -> Result<(), DecompressError> {
    let world: Vec<u8> = read("tests/XBOX360_TU69.bin")?;
    let mut unzipped: Vec<u8> = vec![];
    x_decompress(&world, &mut unzipped)?;
    println!("{:?}", unzipped);
    Ok(())
}
