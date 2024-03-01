use std::ffi::OsStr;
use std::path::PathBuf;

/// an application that is stored in the yard and can be executed
#[derive(Debug, PartialEq)]
pub struct Executable(pub PathBuf);

impl AsRef<OsStr> for Executable {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}
