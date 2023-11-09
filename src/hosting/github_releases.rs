use super::OnlineLocation;
use crate::download::{http_get, Artifact};
use crate::ui::Output;
use crate::Result;

/// a file ("asset" in GitHub lingo) attached to a particular GitHub Release
pub struct GithubReleaseAsset {
    pub organization: &'static str,
    pub repo: &'static str,
    pub version: String,
    pub filename: String,
}

impl OnlineLocation for GithubReleaseAsset {
    fn download(&self, output: &dyn Output) -> Result<Artifact> {
        let url = self.download_url();
        http_get(url, output)
    }
}

impl GithubReleaseAsset {
    pub fn download_url(&self) -> String {
        format!(
            "https://github.com/{organization}/{repo}/releases/download/{version}/{filename}",
            organization = self.organization,
            repo = self.repo,
            version = self.version,
            filename = self.filename,
        )
    }
}
