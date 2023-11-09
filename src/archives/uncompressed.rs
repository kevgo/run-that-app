use crate::ui::output::Output;

use super::Archive;

pub struct Uncompressed {
    pub data: Vec<u8>,
}

impl Archive for Uncompressed {
    fn extract(
        &self,
        _files: Vec<String>,
        target: &std::path::Path,
        output: &dyn Output,
    ) -> crate::error::Result<crate::yard::RunnableApp> {
        todo!()
    }
}
