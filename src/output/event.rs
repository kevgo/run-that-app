use std::borrow::Cow;

use crate::config::AppName;

/// the different events that can result in CLI output
pub enum Event<'a> {
    CpuIdentified { architecture: &'static str },
    OsIdentified { name: &'static str },
    DownloadBegin { app: &'a AppName, url: String },
    DownloadSuccess,
    DownloadNotOnline,
    DownloadNotFound,
    DownloadFail { code: i32 },
    ExtractBegin { archive_type: String },
    ExtractSuccess,
    ExtractFail,
    CompileGoStart { go_path: Cow<'a, str>, args: &'a [&'a str] },
    CompileGoSuccess,
    CompileRustStart { cargo_path: Cow<'a, str>, args: &'a [&'a str] },
    CompileRustSuccess,
}
