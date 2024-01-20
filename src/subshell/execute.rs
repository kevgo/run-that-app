use super::exit_status_to_code;
use crate::error::UserError;
use crate::yard::Executable;
use crate::Result;
use std::process::{Command, ExitCode};
use std::str;

/// Runs the given executable with the given arguments.
/// Streams output to the user's terminal.
pub fn run(Executable(app): Executable, args: &[String]) -> Result<ExitCode> {
    let mut cmd = Command::new(&app);
    cmd.args(args);
    let exit_status = cmd.status().map_err(|err| UserError::CannotExecuteBinary {
        call_signature: format!("{} {}", app.to_string_lossy(), args.join(" ")),
        reason: err.to_string(),
    })?;
    Ok(exit_status_to_code(exit_status))
}

/// Runs the given executable with the given arguments.
/// Captures and returns the output.
pub fn query(Executable(app): Executable, args: &[String]) -> Result<String> {
    let mut cmd = Command::new(&app);
    cmd.args(args);
    let output = cmd.output().map_err(|err| UserError::CannotExecuteBinary {
        call_signature: format!("{} {}", app.to_string_lossy(), args.join(" ")),
        reason: err.to_string(),
    })?;
    let stdout = str::from_utf8(&output.stdout).unwrap_or("");
    let stderr = str::from_utf8(&output.stderr).unwrap_or("");
    Ok(format!("{stdout}{stderr}"))
}

#[cfg(test)]
mod tests {
    mod execute {

        #[test]
        #[cfg(unix)]
        fn unix_success() {
            use crate::subshell::run;
            use crate::yard::Executable;
            use big_s::S;
            use std::io::Write;
            use std::os::unix::fs::PermissionsExt;
            use std::time::Duration;
            use std::{fs, thread};
            let tempdir = tempfile::tempdir().unwrap();
            let executable_path = tempdir.path().join("executable");
            let mut file = fs::File::create(&executable_path).unwrap();
            file.write_all(b"#!/bin/sh\necho hello").unwrap();
            file.set_permissions(fs::Permissions::from_mode(0o744)).unwrap();
            drop(file);
            thread::sleep(Duration::from_millis(10)); // give the OS time to close the file to avoid a flaky test
            let have = run(Executable(executable_path), &[]).unwrap();
            // HACK: is there a better way to compare ExitCode?
            assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(0))"));
        }

        #[test]
        #[cfg(unix)]
        fn unix_error() {
            use crate::filesystem::make_file_executable;
            use crate::subshell::run;
            use crate::yard::Executable;
            use big_s::S;
            use std::fs;
            let tempdir = tempfile::tempdir().unwrap();
            let executable_path = tempdir.path().join("executable");
            fs::write(&executable_path, b"#!/bin/sh\nexit 3").unwrap();
            make_file_executable(&executable_path).unwrap();
            let executable = Executable(executable_path);
            let have = run(executable, &[]).unwrap();
            // HACK: is there a better way to compare ExitCode?
            assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(3))"));
        }

        #[test]
        #[cfg(windows)]
        fn windows_success() {
            use crate::subshell::run;
            use crate::yard::Executable;
            use big_s::S;
            use std::fs;
            let tempdir = tempfile::tempdir().unwrap();
            let executable_path = tempdir.path().join("executable.cmd");
            fs::write(&executable_path, b"echo hello").unwrap();
            let executable = Executable(executable_path);
            let have = run(executable, &[]).unwrap();
            // HACK: is there a better way to compare ExitCode?
            assert_eq!(format!("{have:?}"), S("ExitCode(ExitCode(0))"));
        }

        #[test]
        #[cfg(windows)]
        fn windows_error() {
            use crate::subshell::run;
            use crate::yard::Executable;
            use big_s::S;
            use std::fs;
            let tempdir = tempfile::tempdir().unwrap();
            let executable_path = tempdir.path().join("executable.cmd");
            fs::write(&executable_path, b"EXIT 3").unwrap();
            let executable = Executable(executable_path);
            let have = run(executable, &[]).unwrap();
            // HACK: is there a better way to compare ExitCode?
            assert_eq!(format!("{have:?}"), S("ExitCode(ExitCode(3))"));
        }
    }
}
