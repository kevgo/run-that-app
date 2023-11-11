mod github_releases;

use crate::download::Artifact;
use crate::Output;
use crate::Result;
pub use github_releases::GithubReleaseAsset;

/// an online location containing an application
pub trait OnlineLocation {
    /// downloads the artifact containing the application from this hoster
    fn download(&self, output: &dyn Output) -> Result<Artifact>;
}
