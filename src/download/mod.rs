//! downloading artifacts from the internet

mod http_get;

pub use http_get::http_get;

/// Artifacts are downloaded files from the internet.
/// Typically they are archives containing an application binary.
/// They could also be the binary itself.
pub struct Artifact {
    pub filename: String,
    pub data: Vec<u8>,
}
