use crate::configuration::{self, FILE_NAME};
use colored::Colorize;
use std::path::PathBuf;

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub(crate) type Result<T> = core::result::Result<T, UserError>;

/// errors that are the user's fault and should be displayed to them
#[derive(Debug, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub(crate) enum UserError {
  ArchiveCannotExtract {
    reason: String,
  },
  #[cfg(unix)]
  ArchiveDoesNotContainExecutable {
    expected: PathBuf,
  },
  CannotAccessConfigFile(String),
  CannotCompileRustSource {
    err: String,
  },
  CannotCreateFile {
    filename: &'static str,
    err: String,
  },
  CannotCreateFolder {
    folder: PathBuf,
    reason: String,
  },
  CannotCreateTempDir {
    err: String,
  },
  CannotDeleteFolder {
    folder: PathBuf,
    err: String,
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
  CannotFindExecutable,
  #[cfg(unix)]
  CannotMakeFileExecutable {
    file: PathBuf,
    reason: String,
  },
  CannotOpenSubshellStream,
  CannotParseSemverVersion {
    expression: String,
    reason: String,
  },
  CannotParseSemverRange {
    expression: String,
    reason: String,
  },
  #[cfg(unix)]
  CannotReadFileMetadata {
    err: String,
  },
  CannotReadZipFile {
    err: String,
  },
  #[cfg(unix)]
  CannotSetFilePermissions {
    path: PathBuf,
    err: String,
  },
  CompilationError {
    reason: String,
  },
  CompilationInterupted,
  ConfigFileAlreadyExists,
  DuplicateAppName {
    name: String,
  },
  ExecutableCannotExecute {
    executable: PathBuf,
    err: String,
  },
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
  InvalidConfigFileFormat {
    line_no: usize,
    text: String,
  },
  InvalidNumber,
  InvalidGitHubAPIResponse {
    err: String,
  },
  InvalidRegex {
    regex: String,
    err: String,
  },
  MissingApplication,
  MultipleCommandsGiven,
  NotOnline,
  NoVersionsFound {
    app: String,
  },
  ProcessEmittedOutput {
    cmd: String,
  },
  RegexDoesntMatch,
  RegexHasNoCaptures,
  RunRequestMissingVersion {
    app: String,
  },
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
  pub(crate) fn print(self) {
    match self {
      UserError::ArchiveCannotExtract { reason } => {
        error(&format!("cannot extract the archive: {reason}"));
      }
      #[cfg(unix)]
      UserError::ArchiveDoesNotContainExecutable { expected } => {
        error(&format!("archive does not contain the expected executable: {}", expected.to_string_lossy()));
      }
      UserError::CannotAccessConfigFile(reason) => {
        error(&format!("cannot read the config file: {reason}"));
        desc(&format!("please make sure {} is a file and accessible to you", configuration::FILE_NAME));
      }
      UserError::CannotCompileRustSource { err } => error(&format!("cannot compile Rust source: {err}")),
      UserError::CannotDeleteFolder { folder, err } => error(&format!("cannot delete folder {}: {err}", folder.to_string_lossy())),
      UserError::CannotDetermineCurrentDirectory(reason) => error(&format!("cannot determine the current directory: {reason}")),
      UserError::CannotCreateFile { filename, err } => error(&format!("cannot create file {filename}: {err}")),
      UserError::CannotCreateFolder { folder, reason } => {
        error(&format!("cannot create folder {folder}: {reason}", folder = folder.to_string_lossy()));
        desc("Please check access permissions and try again.");
      }
      UserError::CannotCreateTempDir { err } => error(&format!("cannot create temporary directory: {err}")),
      UserError::CannotDetermineHomeDirectory => error("cannot determine home directory"),
      UserError::CannotDownload { url, reason } => {
        error(&format!("cannot download URL {url}: {reason}"));
        desc("Please try again later.");
      }
      UserError::CannotExecuteBinary { call, reason } => {
        error(&format!("cannot execute \"{call}\":\n{reason}"));
      }
      UserError::CannotFindExecutable => {
        error("cannot locate executable for app.");
        desc("Please report this at https://github.com/kevgo/run-that-app/issues/new and try using an older version until this is fixed.");
      }
      #[cfg(unix)]
      UserError::CannotMakeFileExecutable { file, reason } => {
        error(&format!("Cannot make file {} executable: {reason}", file.to_string_lossy()));
        desc("Please check access permissions and try again.");
      }
      UserError::CannotOpenSubshellStream => error("cannot open subshell stream"),
      UserError::CannotParseSemverVersion { expression, reason } => {
        error(&format!("semver version \"{expression}\" is incorrect: {reason}"));
        desc("Please use exactly three numbers separated by dots, e.g. 1.2.3");
      }
      UserError::CannotParseSemverRange { expression, reason } => {
        error(&format!("semver range \"{expression}\" is incorrect: {reason}"));
        desc("Please use formats described at https://devhints.io/semver.");
      }
      #[cfg(unix)]
      UserError::CannotReadFileMetadata { err } => {
        error(&format!("cannot read file metadata: {err}"));
        desc(
          "This is an issue with your operating system permissions. Please allow the current user to read file permissions for the given path and try again.",
        );
      }
      UserError::CannotReadZipFile { err } => error(&format!("cannot read ZIP file: {err}")),
      #[cfg(unix)]
      UserError::CannotSetFilePermissions { path, err } => {
        error(&format!("cannot write permissions for file {}: {err}", path.to_string_lossy()));
        desc("This is an issue with your operating system permissions. Please allow the current user to change permissions for the given path and try again.");
      }
      UserError::CompilationError { reason } => {
        error(&format!("Compilation error: {reason}"));
      }
      UserError::CompilationInterupted => {
        error("Canceling the compilation");
      }
      UserError::ConfigFileAlreadyExists => {
        error("config file already exists");
        desc(&format!("The file {FILE_NAME} already exists, no changes have been made to it."));
      }
      UserError::DuplicateAppName { name } => {
        error(&format!("duplicate app name found: {name}"));
      }
      UserError::ExecutableCannotExecute { executable, err } => {
        error(&format!("cannot execute {}: {err}", executable.to_string_lossy()));
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
      UserError::InvalidConfigFileFormat { line_no, text } => {
        error("Invalid config file format");
        desc(&format!("{}:{line_no}: {text}", configuration::FILE_NAME));
      }
      UserError::InvalidGitHubAPIResponse { err } => error(&format!("invalid GitHub API response: {err}")),
      UserError::InvalidNumber => {
        error("Invalid number given");
      }
      UserError::InvalidRegex { regex, err } => error(&format!("invalid regex '{regex}': {err}")),
      UserError::MissingApplication => {
        error("missing application");
        desc("Please provide the application to execute");
      }
      UserError::MultipleCommandsGiven => {
        error("multiple commands given");
        desc("Please provide either --which or --available or nothing to run the app, but not both");
      }
      UserError::NotOnline => error("not online"),
      UserError::NoVersionsFound { app } => error(&format!(r#"cannot determine versions for application "{app}""#)),
      UserError::ProcessEmittedOutput { cmd } => {
        error(&format!("process \"{cmd}\" emitted unexpected output"));
      }
      UserError::RegexDoesntMatch => error("this regex doesn't match"),
      UserError::RegexHasNoCaptures => error("regex has no captures"),
      UserError::RunRequestMissingVersion { app } => {
        error(&format!("missing application version for {app}"));
        desc("Please provide the exact version of the app you want to execute in this format: app@1.2.3");
        desc(&format!(
          "You can also create a file {} that defines them using this format: https://asdf-vm.com/manage/configuration.html",
          configuration::FILE_NAME,
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
        error(&format!("Unknown application: {app_name}"));
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

As a workaround, you could install this app in other ways and then add a \"system\" version to .run-that-app.
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

fn error(text: &str) {
  println!("\n{} {}", "ERROR:".red().bold(), text.red().bold());
}

fn desc(text: &str) {
  println!("\n{text}");
}
