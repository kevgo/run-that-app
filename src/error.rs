use std::path::PathBuf;

use colored::Colorize;

/// errors that are the user's fault and should be displayed to them
#[derive(Debug, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub enum UserError {
    CannotDetermineHomeDirectory,
    CannotDetermineOS,
    CannotDownload { url: String, reason: String },
    CannotCreateFolder { folder: PathBuf, reason: String },
    CannotMakeFileExecutable { file: String, reason: String },
    NotOnline,
    RunRequestMissingVersion,
    UnknownApp(String),
    UnknownCliOption(String),
    UnsupportedPlatform,
    UnsupportedCPU(String),
    UnsupportedOS(String),
    // UnsupportedPlatformAndNoGlobalApp {
    //     app_name: String,
    //     platform: Platform,
    // },
    YardRootIsNotFolder { root: PathBuf },
}

impl UserError {
    pub fn print(self) {
        match self {
            UserError::CannotDetermineHomeDirectory => error("cannot determine home directory"),
            UserError::CannotDetermineOS => {
                error("cannot determine the operating system");
                desc("Request support for your platform at https://github.com/kevgo/binstall/issues.");
            }
            UserError::CannotCreateFolder { folder, reason } => {
                error(&format!(
                    "cannot create folder {folder}: {reason}",
                    folder = folder.to_string_lossy()
                ));
                desc("Please check access permissions and try again.");
            }
            UserError::CannotDownload { url, reason } => {
                error(&format!("cannot download URL {url}: {reason}"));
                desc("Please try again later.");
            }
            UserError::CannotMakeFileExecutable { file, reason } => {
                error(&format!("Cannot make file {file} executable: {reason}"));
                desc("Please check access permissions and try again.");
            }
            UserError::NotOnline => error("you seem to be offline"),
            UserError::RunRequestMissingVersion => {
                error("missing the version to install");
                desc("To create a fully reproducible build, please provide the exact version you want to install.");
            }
            UserError::UnknownApp(app_name) => {
                error(&format!("Unknown app: {app_name}"));
                // help::print_installable_apps();
            }
            UserError::UnknownCliOption(option) => {
                error(&format!("Unknown option: {option}"));
                // help::print_options();
            }
            UserError::UnsupportedCPU(name) => {
                error(&format!("Your CPU ({name}) is currently not supported."));
                desc("Request support for your platform at https://github.com/kevgo/binstall/issues.");
            }
            UserError::UnsupportedPlatform => {
                error("This application does not seem to support your platform.");
                desc("It looks like there are no binary versions for this app for your platform.

As a workaround, you could install this app in other ways and then run \"binstall --fallback-to-existing\".
If you are okay moving forward without this app, you can provide the \"--allow-unavailable\" switch and binstall will install a non-functional stub for it.",
                              );
            }
            UserError::UnsupportedOS(name) => {
                error(&format!(
                    "Your operating system ({name}) is currently not supported."
                ));
                desc("Request support for your platform at https://github.com/kevgo/binstall/issues.");
            } // UserError::UnsupportedPlatformAndNoGlobalApp { app_name, platform } => {
            //     error(&format!("This app is not supported on {platform} and I didn't find a globally installed version in your PATH."));
            //     desc(&format!(
            //         "Please make sure that running \"{app_name}\" works and try again."
            //     ));
            // }
            UserError::YardRootIsNotFolder { root } => {
                error("The internal storage has the wrong structure.");
                desc(&format!(
                    "{} should is not a folder. Please delete it and try again.",
                    root.to_string_lossy()
                ));
            }
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
