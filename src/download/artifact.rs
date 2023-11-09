use crate::archives::{self, Archive};
use crate::error::UserError;
use crate::Result;

/// a downloaded file from the internet, usually an archive containing an application binary
pub struct Artifact {
    pub filename: String,
    pub data: Vec<u8>,
}

impl Artifact {
    pub fn to_archive(self) -> Result<Box<dyn Archive>> {
        if self.filename.ends_with(".tar.gz") {
            return Ok(Box::new(archives::TarGz { data: self.data }));
        }
        if self.filename.ends_with(".zip") {
            return Ok(Box::new(archives::Zip { data: self.data }));
        }
        Err(UserError::UnknownArchive(self.filename.to_string()))
    }
}
