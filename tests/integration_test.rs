use std::fs::read;

use lzx_rs::{Status, inflate_listing};

#[test]
fn decompress_xbox_360() -> Result<(), Status> {
    let world: Vec<u8> = read("tests/XBOX360_TU74.dat").map_err(|_| Status::FileError)?;
    let unzipped: Vec<u8> = inflate_listing(world)?;
    println!("{:?}", unzipped);
    Ok(())
}
