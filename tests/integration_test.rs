use std::{fs::read, io};

use lzx_rs::{XmemErr, x_decompress};

#[derive(Debug)]
enum DecompressError {
    Io(io::Error),
    XmemErr(XmemErr),
}

#[test]
fn decompress_xbox_360() -> Result<(), DecompressError> {
    let world: Vec<u8> = read("tests/XBOX360_TU74.dat").map_err(DecompressError::Io)?;
    let mut unzipped: Vec<u8> = vec![];
    x_decompress(&world, &mut unzipped).map_err(DecompressError::XmemErr)?;
    println!("{:?}", unzipped);
    Ok(())
}
