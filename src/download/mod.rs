//! downloading artifacts from the internet

mod artifact;
mod http_get;

pub use artifact::Artifact;
pub use http_get::http_get;
