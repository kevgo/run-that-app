use super::Archive;

/// a .tar.gz file downloaded from the internet, containing an application
pub struct Zip {
    pub data: Vec<u8>,
}

impl Archive for Zip {
    fn extract(
        &self,
        files: Vec<String>,
        target: &std::path::Path,
    ) -> crate::error::Result<crate::yard::RunnableApp> {
        todo!()
    }
}
