use crate::yard::Executable;
use std::process::{Command, ExitCode};

pub fn execute(Executable(app): Executable, args: Vec<String>) -> ExitCode {
    let mut cmd = Command::new(app);
    cmd.args(args);
    let exit_code = cmd.status().unwrap().code().unwrap_or(255);
    ExitCode::from(reduce_exit_status_to_code(exit_code))
}

fn reduce_exit_status_to_code(code: i32) -> u8 {
    if code < 0 {
        return 255;
    }
    if code > 255 {
        return 255;
    }
    u8::try_from(code).unwrap()
}

#[cfg(test)]
mod tests {
    mod execute {

        #[test]
        #[cfg(unix)]
        fn unix_success() {
            use crate::filesystem::make_file_executable;
            use crate::subshell::execute;
            use crate::yard::Executable;
            use big_s::S;
            use std::fs;
            let tempdir = tempfile::tempdir().unwrap();
            let executable_path = tempdir.path().join("executable");
            fs::write(&executable_path, b"#!/bin/sh\necho hello").unwrap();
            make_file_executable(&executable_path).unwrap();
            let runnable_app = Executable(executable_path);
            let have = execute(runnable_app, vec![]);
            // HACK: is there a better way to compare ExitCode?
            assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(0))"));
        }

        #[test]
        #[cfg(unix)]
        fn unix_error() {
            use crate::filesystem::make_file_executable;
            use crate::subshell::execute;
            use crate::yard::Executable;
            use big_s::S;
            use std::fs;
            let tempdir = tempfile::tempdir().unwrap();
            let executable_path = tempdir.path().join("executable");
            fs::write(&executable_path, b"#!/bin/sh\nexit 3").unwrap();
            make_file_executable(&executable_path).unwrap();
            let runnable_app = Executable(executable_path);
            let have = execute(runnable_app, vec![]);
            // HACK: is there a better way to compare ExitCode?
            assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(3))"));
        }

        #[test]
        #[cfg(windows)]
        fn windows_success() {
            use crate::subshell::execute;
            use crate::yard::Executable;
            use big_s::S;
            use std::fs;
            let tempdir = tempfile::tempdir().unwrap();
            let executable_path = tempdir.path().join("executable.cmd");
            fs::write(&executable_path, b"echo hello").unwrap();
            let runnable_app = RunnableApp {
                executable: executable_path,
            };
            let have = execute(runnable_app, vec![]);
            // HACK: is there a better way to compare ExitCode?
            assert_eq!(format!("{have:?}"), S("ExitCode(ExitCode(0))"));
        }

        #[test]
        #[cfg(windows)]
        fn windows_error() {
            use crate::subshell::execute;
            use crate::yard::Executable;
            use big_s::S;
            use std::fs;
            let tempdir = tempfile::tempdir().unwrap();
            let executable_path = tempdir.path().join("executable.cmd");
            fs::write(&executable_path, b"EXIT 3").unwrap();
            let runnable_app = RunnableApp {
                executable: executable_path,
            };
            let have = execute(runnable_app, vec![]);
            // HACK: is there a better way to compare ExitCode?
            assert_eq!(format!("{have:?}"), S("ExitCode(ExitCode(3))"));
        }
    }

    #[test]
    fn reduce_exit_status_to_code() {
        let tests: Vec<(i32, u8)> = vec![
            (i32::MIN, 255),
            (-1, 255),
            (0, 0),
            (1, 1),
            (254, 254),
            (255, 255),
            (256, 255),
            (i32::MAX, 255),
        ];
        for (give, want) in tests {
            let have = super::reduce_exit_status_to_code(give);
            assert_eq!(have, want);
        }
    }
}
