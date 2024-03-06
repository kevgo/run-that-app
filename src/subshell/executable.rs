use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

/// an application that is stored in the yard and can be executed
#[derive(Debug, PartialEq)]
pub struct Executable(pub PathBuf);

impl AsRef<OsStr> for Executable {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

impl Executable {
    /// runs this executable with the given args and returns the output it produced
    pub fn run_output(&self, arg: &str) -> String {
        let mut cmd = Command::new(self);
        cmd.arg(arg);
        let Ok(output) = cmd.output() else {
            return String::new();
        };
        String::from_utf8(output.stdout).unwrap_or_default()
    }

    /// runs this executable with the given args and returns the output it produced
    pub fn run_output_args(&self, args: &[&str]) -> String {
        let mut cmd = Command::new(self);
        cmd.args(args);
        let Ok(output) = cmd.output() else {
            return String::new();
        };
        String::from_utf8(output.stdout).unwrap_or_default()
    }
}
