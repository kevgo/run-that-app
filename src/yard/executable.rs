use std::path::PathBuf;

/// an application that is stored in the yard and can be executed
#[derive(Debug, PartialEq)]
pub struct Executable {
    pub path: PathBuf,
}
