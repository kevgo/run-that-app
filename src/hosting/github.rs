use super::Hoster;
use crate::detect::Platform;
use crate::download::Artifact;
use crate::Result;

pub struct GitHub {
    pub organization: String,
    pub repo: String,
}

impl Hoster for GitHub {
    fn download(&self, platform: &Platform) -> Result<Artifact> {
        todo!()
    }

    fn homepage(&self) -> String {
        format!("https://github.com/{}/{}", self.organization, self.repo)
    }
}
