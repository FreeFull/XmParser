use error::XmResult;
use error::XmError::InvalidFile;

pub fn match_constant(buffer: &[u8], constant: &[u8]) -> XmResult<()> {
    if buffer[..constant.len()] == constant {
        Ok(())
    } else {
        Err(InvalidFile)
    }
}
