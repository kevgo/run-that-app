use crate::archives::{self, Archive};

/// Artifacts are downloaded files from the internet.
/// Typically they are archives containing an application binary.
/// They could also be the binary itself.
pub struct Artifact {
    pub filename: String,
    pub data: Vec<u8>,
}

impl Artifact {
    pub fn to_archive(self) -> Box<dyn Archive> {
        if self.filename.ends_with(".tar.gz") {
            return Box::new(archives::TarGz { data: self.data });
        }
        if self.filename.ends_with(".zip") {
            return Box::new(archives::Zip { data: self.data });
        }
        Box::new(archives::Uncompressed { data: self.data })
    }
}
