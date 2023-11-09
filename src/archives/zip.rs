use super::Archive;
use crate::ui::output::Output;
use std::path::PathBuf;

/// a .tar.gz file downloaded from the internet, containing an application
pub struct Zip {
    pub data: Vec<u8>,
}

impl Archive for Zip {
    fn extract(
        &self,
        path_in_archive: String,
        path_on_disk: PathBuf,
        output: &dyn Output,
    ) -> crate::error::Result<crate::yard::RunnableApp> {
        todo!()
    }
}
