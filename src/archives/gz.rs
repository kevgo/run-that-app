use super::Archive;
use crate::applications::ApplicationName;
use crate::error::{Result, UserError};
use crate::executables::ExecutableNameUnix;
use crate::filesystem;
use crate::logging::{Event, Log};
use crate::platform::Platform;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io;
use std::path::Path;

/// a .gz file downloaded from the internet, containing a single executable
pub struct Gz {
  pub data: Vec<u8>,
}

impl Archive for Gz {
  fn extract_all(&self, target_dir: &Path, platform: Platform, log: Log, app: &ApplicationName) -> Result<()> {
    log(Event::ArchiveExtractBegin { archive_type: "gz" });
    let executable_name_unix = ExecutableNameUnix::from(app.as_str());
    let executable_name_platform = executable_name_unix.platform_path(platform.os);
    let output_path = &target_dir.join(executable_name_platform.as_ref());
    let mut gz_decoder = GzDecoder::new(io::Cursor::new(&self.data));
    match File::create(output_path) {
      Ok(mut file) => {
        if let Err(err) = io::copy(&mut gz_decoder, &mut file) {
          log(Event::ArchiveExtractFailed { err: &err });
          return Err(UserError::ArchiveCannotExtract { reason: err.to_string() });
        }
        drop(file); // close file before setting permissions
        filesystem::set_executable_bit(output_path);
        log(Event::ArchiveExtractSuccess);
        Ok(())
      }
      Err(err) => {
        log(Event::ArchiveExtractFailed { err: &err });
        Err(UserError::ArchiveCannotExtract { reason: err.to_string() })
      }
    }
  }
}
