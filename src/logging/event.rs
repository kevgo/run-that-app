use crate::applications::ApplicationName;
use crate::configuration::Version;
use crate::download::Url;
use crate::executables::ExecutableNamePlatform;
use crate::installation::Method;
use std::borrow::Cow;
use std::fmt::Display;
use std::path::Path;

/// the different events that can result in CLI output
pub enum Event<'a> {
  AnalyzeExecutableBegin {
    cmd: &'a str,
    args: &'a [&'a str],
  },
  ArchiveExtractBegin {
    archive_type: &'a str,
  },
  ArchiveExtractSuccess,
  ArchiveExtractFailed {
    err: &'a dyn Display,
  },
  CompileGoBegin {
    go_path: Cow<'a, str>,
    args: &'a [&'a str],
  },
  CompileGoSuccess,
  CompileGoFailed,
  CompileRustStart {
    cargo_path: &'a Path,
    args: &'a [String],
  },
  CompileRustSuccess,
  CompileRustFailed,
  DownloadBegin {
    app: &'a ApplicationName,
    version: &'a Version,
    url: &'a Url,
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
    err: &'a dyn Display,
  },
  GitHubApiRequestBegin {
    url: &'a str,
  },
  GitHubApiRequestFail {
    err: &'a dyn Display,
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
    app: &'a ApplicationName,
  },
  IntegrationTestDeterminedVersion {
    version: &'a Version,
  },
  IntegrationTestNewInstallMethod {
    app: &'a ApplicationName,
    method: &'a Method,
    version: &'a Version,
  },
  LockAcquireBegin {
    app: &'a ApplicationName,
  },
  LockAcquireSuccess,
  LockRelease {
    app: &'a ApplicationName,
  },
  NotOnline,
  UpdateBegin {
    app: &'a ApplicationName,
  },
  UpdateNewVersion {
    app: &'a ApplicationName,
    old_version: &'a Version,
    new_version: &'a Version,
  },
  UpdateAlreadyNewest {
    app: &'a ApplicationName,
  },
  YardCheckExistingAppBegin {
    path: &'a Path,
  },
  YardCheckExistingAppFound,
  YardCheckExistingAppNotFound,
}
