use crate::config::AppName;
use crate::logger::{Event, Log};
use crate::{Result, UserError};

/// downloads the artifact at the given URL
pub fn artifact(url: String, app: &AppName, log: Log) -> Result<Option<Artifact>> {
    log(Event::DownloadBegin { app, url: &url });
    let Ok(response) = minreq::get(&url).send() else {
        log(Event::NotOnline);
        return Err(UserError::NotOnline);
    };
    if response.status_code == 404 {
        log(Event::DownloadNotFound);
        return Ok(None);
    }
    if response.status_code != 200 {
        log(Event::DownloadFail { code: response.status_code });
        return Err(UserError::CannotDownload {
            reason: response.reason_phrase,
            url: url.to_string(),
        });
    }
    log(Event::DownloadSuccess);
    Ok(Some(Artifact {
        filename: url,
        data: response.into_bytes(),
    }))
}

/// An artifacts is a file containing an application, downloaded from the internet.
/// An artifact could be an archive containing the application binary (and other files),
/// or the uncompressed application binary itself.
pub struct Artifact {
    pub filename: String,
    pub data: Vec<u8>,
}
