use super::Archive;
use crate::applications::ApplicationName;
use crate::error::{Result, UserError};
use crate::filesystem;
use crate::logging::{Event, Log};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io;
use std::path::Path;

/// a .gz file downloaded from the internet, containing a single executable
pub(crate) struct Gz {
  pub(crate) data: Vec<u8>,
}

impl Archive for Gz {
  fn extract_all(&self, target_dir: &Path, log: Log, app: &ApplicationName) -> Result<()> {
    log(Event::ArchiveExtractBegin { archive_type: "gz" });
    let output_path = target_dir.join(app);
    let mut gz_decoder = GzDecoder::new(io::Cursor::new(&self.data));
    match File::create(&output_path) {
      Ok(mut file) => {
        if let Err(err) = io::copy(&mut gz_decoder, &mut file) {
          log(Event::ArchiveExtractFailed { err: err.to_string() });
          return Err(UserError::ArchiveCannotExtract { reason: err.to_string() });
        }
        drop(file); // close file before setting permissions
        filesystem::make_executable(&output_path)?;
        log(Event::ArchiveExtractSuccess);
        Ok(())
      }
      Err(err) => {
        log(Event::ArchiveExtractFailed { err: err.to_string() });
        Err(UserError::ArchiveCannotExtract { reason: err.to_string() })
      }
    }
  }
}
