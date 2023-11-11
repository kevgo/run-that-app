use crate::yard::RunnableApp;
use std::process::{Command, ExitCode};

pub fn execute(app: RunnableApp, args: Vec<String>) -> ExitCode {
    let mut cmd = Command::new(app.executable);
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
