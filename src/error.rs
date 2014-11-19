use std::io::IoError;
use std::error::{Error, FromError};

use self::XmError as XE;

pub enum XmError {
    InvalidFile,
    WrongVersion(u8,u8),
    IoError(::std::io::IoError),
}

pub type XmResult<T> = Result<T,XmError>;

impl Error for XmError {
    fn description(&self) -> &str {
        match *self {
            XE::InvalidFile => "The file isn't a valid XM file.",
            XE::WrongVersion(..) => "Expected version 01.04",
            XE::IoError(ref err) => err.description(),
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            XE::WrongVersion(major, minor) => Some(format!("Got version {}.{}", major, minor)),
            _ => None
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            XE::IoError(ref err) => Some(err as &Error),
            _ => None
        }
    }
}

impl FromError<IoError> for XmError {
    fn from_error(err: IoError) -> XmError {
        XE::IoError(err)
    }
}
