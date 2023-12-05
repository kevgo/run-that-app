use super::HostingPlatform;
use crate::download::http_get;
use crate::Output;
use crate::Result;

pub struct GitHub {
    organization: &'static str,
    repo: &'static str,
}

impl HostingPlatform for GitHub {
    fn versions(&self, output: &dyn Output) -> Result<Vec<String>> {
        let url = format!("https://api.github.com/repos/{org}/{repo}/releases", org = self.organization, repo = self.repo);
        let Some(data) = http_get(&url, output)? else {
            panic!("GitHub API 404");
        };
        println!("{}", String::from_utf8_lossy(&data));
        Ok(vec![])
    }
}
