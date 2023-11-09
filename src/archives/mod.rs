//! An archive is a compressed file containing an application.

mod tar_gz;
mod uncompressed;
mod zip;

use crate::ui::output::{self, Output};
use crate::yard::RunnableApp;
use crate::Result;
use std::path::Path;
pub use tar_gz::TarGz;
pub use uncompressed::Uncompressed;
pub use zip::Zip;

pub trait Archive {
    fn extract(
        &self,
        files: Vec<String>,
        target: &Path,
        output: &dyn Output,
    ) -> Result<RunnableApp>;
}
