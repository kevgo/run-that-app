use colored::Colorize;

/// errors that are the user's fault and should be displayed to them
#[derive(Debug, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub enum UserError {
    CannotDetermineCPU,
    CannotDetermineOS,
    CannotDownload { url: String, reason: String },
    DuplicateRunRequest,
    InstallRequestMissingVersion,
    NotOnline,
    UnknownArchive(String),
    UnknownApp(String),
    UnknownCliOption(String),
    // UnsupportedPlatform {
    //     app_name: String,
    //     platform: Platform,
    // },
    // UnsupportedPlatformAndNoGlobalApp {
    //     app_name: String,
    //     platform: Platform,
    // },
}

impl UserError {
    pub fn print(self) {
        match self {
            UserError::CannotDetermineCPU => {
                error("cannot determine the CPU");
                desc("Request support for your platform at https://github.com/kevgo/binstall/issues.");
            }
            UserError::CannotDetermineOS => {
                error("cannot determine the operating system");
                desc("Request support for your platform at https://github.com/kevgo/binstall/issues.");
            }
            UserError::CannotDownload { url, reason } => {
                error(&format!("cannot download URL {url}: {reason}"));
                desc("Please try again later.");
            }
            UserError::DuplicateRunRequest => error("I can only run one application at a time"),
            UserError::InstallRequestMissingVersion => {
                error("missing the version to install");
                desc("To create a fully reproducible build, please provide the exact version you want to install.");
            }
            UserError::NotOnline => error("you seem to be offline"),
            UserError::UnknownArchive(filename) => {
                error(&format!("Unknown archive type: {filename}"));
                desc("This is a bug in binstall. Please report it at https://github.com/kevgo/binstall/issues.");
            }
            UserError::UnknownApp(app_name) => {
                error(&format!("Unknown app: {app_name}"));
                // help::print_installable_apps();
            }
            UserError::UnknownCliOption(option) => {
                error(&format!("Unknown option: {option}"));
                // help::print_options();
            } //             UserError::UnsupportedPlatform { app_name, platform } => {
              //                 error(&format!(
              //                     "{app_name} does not seem to support this platform ({platform}).",
              //                 ));
              //                 desc(
              //                     "It looks like there are no binary versions for this app for your platform.

              // As a workaround, you could install this app in other ways and then run \"binstall --fallback-to-existing\".
              // If you are okay moving forward without this app, you can provide the \"--allow-unavailable\" switch and binstall will install a non-functional stub for it.",
              //                 );
              //             }
              //             UserError::UnsupportedPlatformAndNoGlobalApp { app_name, platform } => {
              //                 error(&format!("This app is not supported on {platform} and I didn't find a globally installed version in your PATH."));
              //                 desc(&format!(
              //                     "Please make sure that running \"{app_name}\" works and try again."
              //                 ));
              //             }
        }
    }
}

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = core::result::Result<T, UserError>;

fn error(text: &str) {
    println!("{} {}", "ERROR:".red(), text.red());
}

fn desc(text: &str) {
    println!("\n{text}");
}
