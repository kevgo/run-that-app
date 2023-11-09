use crate::ui::output::Output;

use super::Archive;

pub struct Uncompressed {
    pub data: Vec<u8>,
}

impl Archive for Uncompressed {
    fn extract(
        &self,
        path_in_archive: String,
        target: &std::path::Path,
        output: &dyn Output,
    ) -> crate::error::Result<crate::yard::RunnableApp> {
        todo!()
    }
}
