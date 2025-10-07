use super::URL;
use crate::applications::ApplicationName;
use crate::error::{Result, UserError};
use crate::logging::{Event, Log};

/// downloads the artifact at the given URL
pub(crate) fn artifact(url: &URL, app: &ApplicationName, optional: bool, log: Log) -> Result<Option<Artifact>> {
  log(Event::DownloadBegin { app, url });
  let Ok(response) = minreq::get(url.as_ref()).send() else {
    log(Event::NotOnline);
    return Err(UserError::NotOnline);
  };
  if response.status_code == 404 {
    log(Event::DownloadNotFound { is_optional: optional });
    return Ok(None);
  }
  if response.status_code != 200 {
    log(Event::DownloadFail { code: response.status_code });
    return Err(UserError::CannotDownload {
      reason: response.reason_phrase,
      url: url.to_owned(),
    });
  }
  log(Event::DownloadSuccess);
  Ok(Some(Artifact {
    filename: url.to_string(),
    data: response.into_bytes(),
  }))
}

/// An artifacts is a file containing an application, downloaded from the internet.
/// An artifact could be an archive containing the application binary (and other files),
/// or the uncompressed application binary itself.
pub(crate) struct Artifact {
  pub(crate) filename: String,
  pub(crate) data: Vec<u8>,
}
