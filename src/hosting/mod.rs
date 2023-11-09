//! the various hosting platforms from which we can download executables

mod github;

use crate::detect::Platform;
use crate::download::Artifact;
use crate::Result;
pub use github::GitHub;

pub trait Hoster {
    fn download(&self, platform: &Platform) -> Result<Artifact>;
    fn homepage(&self) -> String;
}
