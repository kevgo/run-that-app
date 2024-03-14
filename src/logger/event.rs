use crate::config::{AppName, Version};
use std::borrow::Cow;
use std::path::Path;

/// the different events that can result in CLI output
pub enum Event<'a> {
    AnalyzeExecutableCall { cmd: &'a str, args: &'a [&'a str] },
    AnalyzeExecutableError { err: String },

    ArchiveExtractBegin { archive_type: &'a str },
    ArchiveExtractSuccess,
    ArchiveExtractFailed { err: String },

    CompileGoBegin { go_path: Cow<'a, str>, args: &'a [&'a str] },
    CompileGoSuccess,
    CompileGoFailed,

    CompileRustStart { cargo_path: &'a Path, args: &'a [&'a str] },
    CompileRustSuccess,
    CompileRustFailed,

    DownloadBegin { app: &'a AppName, url: &'a str },
    DownloadSuccess,
    DownloadNotFound,
    DownloadFail { code: i32 },

    ExecutableInstallSaveBegin,
    ExecutableInstallSaveSuccess,
    ExecutableInstallSaveFail { err: String },

    GitHubApiRequestBegin { url: &'a str },
    GitHubApiRequestFail { err: String },
    GitHubApiRequestSuccess,

    GlobalInstallSearch { binary: &'a str },
    GlobalInstallFound { path: &'a Path },
    GlobalInstallMatchingVersion { range: &'a semver::VersionReq, version: Option<&'a Version> },
    GlobalInstallMismatchingVersion { range: &'a semver::VersionReq, version: Option<&'a Version> },
    GlobalInstallNotFound,
    GlobalInstallNotIdentified,

    IdentifiedCpu { architecture: &'static str },
    IdentifiedOs { name: &'static str },

    NotOnline,

    UpdateBegin { app: &'a AppName },
    UpdateNewVersion { old_version: &'a Version, new_version: &'a Version },
    UpdateAlreadyNewest { app: &'a AppName },

    YardExistingAppCheckBegin { path: &'a Path },
    YardExistingAppCheckFound,
    YardExistingAppCheckNotFound,
}
