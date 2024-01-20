use super::FILE_NAME;
use crate::Result;
use crate::UserError;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Write;

pub fn create() -> Result<()> {
    let mut file = match OpenOptions::new().write(true).create_new(true).open(FILE_NAME) {
        Ok(file) => file,
        Err(err) => {
            if err.kind() == ErrorKind::AlreadyExists {
                return Err(UserError::ConfigFileAlreadyExists);
            }
            panic!("{}", err);
        }
    };
    file.write_all(CONFIG_TEXT.as_bytes())
        .map_err(|err| UserError::CannotAccessConfigFile(err.to_string()))
}

const CONFIG_TEXT: &str = "\
# actionlint 1.2.26
# gh 2.39.1
";
