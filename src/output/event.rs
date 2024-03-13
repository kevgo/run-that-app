use crate::config::{AppName, Version};
use std::borrow::Cow;
use std::path::Path;

/// the different events that can result in CLI output
pub enum Event<'a> {
    ArchiveExtractBegin {
        archive_type: &'a str,
    },
    ArchiveExtractSuccess,
    ArchiveExtractFailed {
        err: String,
    },
    GitHubApiRequestBegin {
        url: &'a str,
    },
    GitHubApiRequestFail {
        err: String,
    },
    GitHubApiRequestSuccess,
    CpuIdentified {
        architecture: &'static str,
    },
    OsIdentified {
        name: &'static str,
    },
    DownloadBegin {
        app: &'a AppName,
        url: &'a str,
    },
    DownloadSuccess,
    GlobalInstallSearch {
        binary: &'a str,
    },
    GlobalInstallFound {
        path: &'a Path,
    },
    GlobalInstallMatchingVersion {
        version_range: &'a semver::VersionReq,
        version: Option<&'a Version>,
    },
    GlobalInstallMismatchingVersion {
        version_restriction: &'a semver::VersionReq,
        actual_version: Option<&'a Version>,
    },
    GlobalInstallNotFound,
    GlobalInstallNotIdentified,
    DownloadNotFound,
    DownloadFail {
        code: i32,
    },
    ExecutableInstallSave,
    ExecutableInstallSaveSuccess,
    ExecutableInstallSaveFail {
        err: String,
    },
    CompileGoStart {
        go_path: Cow<'a, str>,
        args: &'a [&'a str],
    },
    CompileGoSuccess,
    CompileRustStart {
        cargo_path: &'a Path,
        args: &'a [&'a str],
    },
    CompileRustSuccess,
    NotOnline,
    UpdateBegin {
        app: &'a AppName,
    },
    UpdateNewVersion {
        old_version: &'a Version,
        new_version: &'a Version,
    },
    UpdateAlreadyNewest {
        app: &'a AppName,
    },
}
