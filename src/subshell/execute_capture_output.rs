use super::Executable;
use std::process::Command;

pub fn execute_capture_output(executable: &Executable, arg: &str) -> Option<String> {
    let mut cmd = Command::new(executable);
    cmd.arg(arg);
    let Ok(output) = cmd.output() else {
        return None;
    };
    let Ok(output) = String::from_utf8(output.stdout) else {
        return None;
    };
    Some(output)
}
