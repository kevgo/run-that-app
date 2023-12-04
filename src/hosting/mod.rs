use crate::Output;
use crate::Result;

mod github;

pub trait HostingPlatform {
    /// provides the 5 latest versions of the product at the given
    fn versions(&self, output: &dyn Output) -> Result<Vec<String>>;
}
