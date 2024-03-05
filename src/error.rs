use crate::config::{self, FILE_NAME};
use colored::Colorize;
use std::path::PathBuf;

/// errors that are the user's fault and should be displayed to them
#[derive(Debug, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub enum UserError {
    CannotAccessConfigFile(String),
    CannotCreateFolder {
        folder: PathBuf,
        reason: String,
    },
    CannotDetermineCurrentDirectory(String),
    CannotDetermineHomeDirectory,
    CannotDownload {
        url: String,
        reason: String,
    },
    CannotExecuteBinary {
        call: String,
        reason: String,
    },
    #[cfg(unix)]
    CannotMakeFileExecutable {
        file: String,
        reason: String,
    },
    ConfigFileAlreadyExists,
    GitHubReleasesApiProblem {
        problem: String,
        payload: String,
    },
    GitHubTagsApiProblem {
        problem: String,
        payload: String,
    },
    GoCompilationFailed,
    GoNoPermission,
    GoNotInstalled,
    InvalidConfigFileFormat {
        line_no: usize,
        text: String,
    },
    InvalidNumber,
    MissingApplication,
    MultipleCommandsGiven,
    NotOnline,
    ProcessEmittedOutput {
        cmd: String,
    },
    RunRequestMissingVersion,
    RustCompilationFailed,
    RustNotInstalled,
    RustNoPermission,
    UnknownApp(String),
    UnknownArchive(String),
    UnknownCliOption(String),
    UnsupportedPlatform,
    UnsupportedCPU(String),
    UnsupportedOS(String),
    YardRootIsNotFolder {
        root: PathBuf,
    },
    YardAccessDenied {
        msg: String,
        path: PathBuf,
    },
}

impl UserError {
    #[allow(clippy::too_many_lines)]
    pub fn print(self) {
        match self {
            UserError::CannotAccessConfigFile(reason) => {
                error(&format!("cannot read the config file: {reason}"));
                desc(&format!("please make sure {} is a file and accessible to you", config::FILE_NAME,));
            }
            UserError::CannotDetermineCurrentDirectory(reason) => error(&format!("cannot determine the current directory: {reason}")),
            UserError::CannotCreateFolder { folder, reason } => {
                error(&format!("cannot create folder {folder}: {reason}", folder = folder.to_string_lossy()));
                desc("Please check access permissions and try again.");
            }
            UserError::CannotDetermineHomeDirectory => error("cannot determine home directory"),
            UserError::CannotDownload { url, reason } => {
                error(&format!("cannot download URL {url}: {reason}"));
                desc("Please try again later.");
            }
            UserError::CannotExecuteBinary { call, reason } => {
                error(&format!("cannot execute \"{call}\":\n{reason}"));
            }
            #[cfg(unix)]
            UserError::CannotMakeFileExecutable { file, reason } => {
                error(&format!("Cannot make file {file} executable: {reason}"));
                desc("Please check access permissions and try again.");
            }
            UserError::ConfigFileAlreadyExists => {
                error("config file already exists");
                desc(&format!("The file {FILE_NAME} already exists, no changes have been made to it."));
            }
            UserError::GitHubReleasesApiProblem { problem, payload } => {
                error(&format!("Problem with the GitHub Releases API: {problem}"));
                desc(&payload);
            }
            UserError::GitHubTagsApiProblem { problem, payload } => {
                error(&format!("Problem with the GitHub Tags API: {problem}"));
                desc(&payload);
            }
            UserError::GoCompilationFailed => {
                error("Compilation from Go source failed.");
                desc("Please see the error output above and try again with a different version.");
            }
            UserError::GoNoPermission => error("No permission to execute the Go compiler"),
            UserError::GoNotInstalled => {
                error("The Go compiler is not installed");
                desc("Installation instructions: https://go.dev/dl");
            }
            UserError::InvalidConfigFileFormat { line_no, text } => {
                error("Invalid config file format");
                desc(&format!("{}:{line_no}: {text}", config::FILE_NAME));
            }
            UserError::InvalidNumber => {
                error("Invalid number given");
            }
            UserError::MissingApplication => {
                error("missing application");
                desc("Please provide the application to execute");
            }
            UserError::MultipleCommandsGiven => {
                error("multiple commands given");
                desc("Please provide either --which or --available or nothing to run the app, but not both");
            }
            UserError::NotOnline => error("not online"),
            UserError::ProcessEmittedOutput { cmd } => {
                error(&format!("process \"{cmd}\" emitted unexpected output"));
            }
            UserError::RunRequestMissingVersion => {
                error("missing application version");
                desc("Please provide the exact version of the app you want to execute in this format: app@1.2.3");
                desc(&format!(
                    "You can also create a file {} that defines them using this format: https://asdf-vm.com/manage/configuration.html",
                    config::FILE_NAME,
                ));
            }
            UserError::RustCompilationFailed => {
                error("Compilation from Rust source failed.");
                desc("Please see the error output above and try again with a different version.");
            }
            UserError::RustNoPermission => error("No permission to execute the Rust toolchain"),
            UserError::RustNotInstalled => {
                error("Rust is not installed.");
                desc("Please install Rust via https://rustup.rs and try again.");
            }
            UserError::UnknownApp(app_name) => {
                error(&format!("Unknown app: {app_name}"));
                // help::print_installable_apps();
            }
            UserError::UnknownArchive(filename) => {
                error(&format!("unknown archive type: {filename}"));
            }
            UserError::UnknownCliOption(option) => {
                error(&format!("Unknown option: {option}"));
                // help::print_options();
            }
            UserError::UnsupportedCPU(name) => {
                error(&format!("Your CPU ({name}) is currently not supported."));
                desc("Request support for your platform at https://github.com/kevgo/run-that-app/issues.");
            }
            UserError::UnsupportedOS(name) => {
                error(&format!("Your operating system ({name}) is currently not supported."));
                desc("Request support for your platform at https://github.com/kevgo/run-that-app/issues.");
            } // UserError::UnsupportedPlatformAndNoGlobalApp { app_name, platform } => {
            UserError::UnsupportedPlatform => {
                error("This application does not seem to support your platform.");
                desc(
                    "It looks like there are no binary versions for this app for your platform.

As a workaround, you could install this app in other ways and then add a \"system\" version to .tool-versions.
If you are okay moving forward without this app, you can provide the \"--optional\" switch and run-that-app will ignore this failure.",
                );
            }
            UserError::YardAccessDenied { msg, path } => {
                error(&format!("Access to the Yard denied: {msg}"));
                desc(&format!("Make sure the folder {} is accessible to you", path.to_string_lossy()));
            }
            UserError::YardRootIsNotFolder { root } => {
                error("The internal storage has the wrong structure.");
                desc(&format!("{} should is not a folder. Please delete it and try again.", root.to_string_lossy()));
            }
        }
    }
}

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = core::result::Result<T, UserError>;

fn error(text: &str) {
    println!("\n{} {}", "ERROR:".red().bold(), text.red().bold());
}

fn desc(text: &str) {
    println!("\n{text}");
}
