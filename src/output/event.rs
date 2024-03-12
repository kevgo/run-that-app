use crate::config::AppName;

/// the different events that can result in CLI output
pub enum Event {
    DownloadBegin { app: AppName, url: String },
    DownloadSuccess,
    DownloadFail,
    ExtractBegin { archive: String },
    ExtractSuccess,
    ExtractFail,
}
