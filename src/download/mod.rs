//! downloading artifacts from the internet

mod http_get;

pub use http_get::http_get;

/// An artifacts is a file containing an application, downloaded from the internet.
/// An artifact could be an archive containing the application binary amongst other files,
/// or the application binary itself.
pub struct Artifact {
    pub filename: String,
    pub data: Vec<u8>,
}
