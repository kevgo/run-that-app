use crate::Result;

pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Args> {
    let _skipped_binary_name = args.next();
    Ok(Args {
        log: None,
        command: Command::DisplayHelp,
    })
}

#[derive(Debug, PartialEq)]
pub struct Args {
    pub log: Option<String>,
    pub command: Command,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    RunApp { name: String, version: String },
    DisplayHelp,
    DisplayVersion,
}

#[cfg(test)]
mod tests {
    use super::Args;
    use super::Command;

    #[test]
    fn no_arguments() {
        let args = vec!["run-that-app"].into_iter().map(ToString::to_string);
        let have = super::parse(args).unwrap();
        let want = Args {
            log: None,
            command: Command::DisplayHelp,
        };
        pretty::assert_eq!(have, want);
    }
}
