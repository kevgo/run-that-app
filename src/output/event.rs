use std::borrow::Cow;
use std::path::Path;

use crate::apps::AnalyzeResult;
use crate::config::{AppName, Version};
use crate::subshell::Executable;

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
    GlobalInstallNotFound,
    GlobalInstallAnalyzed {
        executable: &'a Executable,
        app_name: &'a AppName,
        result: &'a AnalyzeResult,
    },
    GitHubApiRequestNotOnline,
    DownloadNotFound,
    DownloadFail {
        code: i32,
    },
    ExecutableInstallSave,
    ExecutableInstallSaveSuccess,
    ExecutableInstallSaveFail,
    ExtractBegin {
        archive_type: String,
    },
    ExtractSuccess,
    ExtractFail,
    CompileGoStart {
        go_path: Cow<'a, str>,
        args: &'a [&'a str],
    },
    CompileGoSuccess,
    CompileRustStart {
        cargo_path: Cow<'a, str>,
        args: &'a [&'a str],
    },
    CompileRustSuccess,
    UpdateBegin {
        app: &'a AppName,
    },
    UpdateNewVersion {
        app: &'a AppName,
        old_version: &'a Version,
        new_versin: &'a Version,
    },
    UpdateAlreadyNewest {
        app: &'a AppName,
    },
}
