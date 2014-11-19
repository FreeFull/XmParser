#![feature(slicing_syntax)]
#![feature(globs)]

use std::io::File;
use std::slice::bytes::copy_memory;
use constants::{HEADER};
use utils::match_constant;
pub use error::{XmError, XmResult};

mod constants;
mod utils;
pub mod error;

pub struct XmModule {
    pub module_name: [u8, ..20],
    pub tracker_name: [u8, ..20],
}

pub fn parse(path: &Path) -> XmResult<XmModule> {
    let mut buffer = [0u8, ..500];
    let mut file = try!(File::open(path));
    try!(file.read(buffer[mut ..HEADER.len()]));
    try!(match_constant(&buffer, HEADER));

    try!(file.read(buffer[mut ..20]));
    let mut module_name = [0, ..20];
    copy_memory(module_name[mut], buffer[..20]);

    try!(file.read(buffer[mut ..1]));
    try!(match_constant(&buffer, b"\x1A"));

    try!(file.read(buffer[mut ..20]));
    let mut tracker_name = [0, ..20];
    copy_memory(tracker_name[mut], buffer[..20]);

    // Version check.
    try!(file.read(buffer[mut ..2]));
    if buffer[..2] != &[1,4] {
        return Err(XmError::WrongVersion(buffer[0],buffer[1]));
    }



    Ok(XmModule {
        module_name: module_name,
        tracker_name: tracker_name,
    })
}
