use crate::configuration::{ApplicationName, Version};
use crate::installation::Method;
use crate::run::ExecutableNamePlatform;
use std::borrow::Cow;
use std::path::Path;

/// the different events that can result in CLI output
pub(crate) enum Event<'a> {
  AnalyzeExecutableBegin {
    cmd: &'a str,
    args: &'a [&'a str],
  },
  AnalyzeExecutableError {
    err: String,
  },
  ArchiveExtractBegin {
    archive_type: &'a str,
  },
  ArchiveExtractSuccess,
  ArchiveExtractFailed {
    err: String,
  },
  CompileGoBegin {
    go_path: Cow<'a, str>,
    args: &'a [&'a str],
  },
  CompileGoSuccess,
  CompileGoFailed,
  CompileRustStart {
    cargo_path: &'a Path,
    args: &'a [&'a str],
  },
  CompileRustSuccess,
  CompileRustFailed,
  DownloadBegin {
    app: &'a ApplicationName<'a>,
    url: &'a str,
  },
  DownloadSuccess,
  DownloadNotFound {
    is_optional: bool,
  },
  DownloadFail {
    code: i32,
  },
  ExecutableInstallSaveBegin,
  ExecutableInstallSaveSuccess,
  ExecutableInstallSaveFail {
    err: String,
  },
  GitHubApiRequestBegin {
    url: &'a str,
  },
  GitHubApiRequestFail {
    err: String,
  },
  GitHubApiRequestSuccess,
  GlobalInstallSearch {
    binary: &'a ExecutableNamePlatform,
  },
  GlobalInstallFound {
    path: &'a Path,
  },
  GlobalInstallMatchingVersion {
    range: &'a semver::VersionReq,
    version: Option<&'a Version>,
  },
  GlobalInstallMismatchingVersion {
    range: &'a semver::VersionReq,
    version: Option<&'a Version>,
  },
  GlobalInstallNotFound,
  GlobalInstallNotIdentified,
  IdentifiedCpu {
    architecture: &'static str,
  },
  IdentifiedOs {
    name: &'static str,
  },
  IntegrationTestNewApp {
    app: &'a ApplicationName<'a>,
  },
  IntegrationTestDeterminedVersion {
    version: &'a Version,
  },
  IntegrationTestNewInstallMethod {
    app: &'a str,
    method: &'a Method,
    version: &'a Version,
  },
  #[cfg(unix)]
  MakeExecutable {
    file: &'a Path,
  },
  NotOnline,
  UpdateBegin {
    app: &'a ApplicationName<'a>,
  },
  UpdateNewVersion {
    old_version: &'a Version,
    new_version: &'a Version,
  },
  UpdateAlreadyNewest,
  YardCheckExistingAppBegin {
    path: &'a Path,
  },
  YardCheckExistingAppFound,
  YardCheckExistingAppNotFound,
}
