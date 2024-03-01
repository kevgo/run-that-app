
/// newtype for strings containing a version
#[derive(Debug, Default, PartialEq)]
pub struct Version(String);

impl From<&str> for Version{
    fn from(text: &str) -> Self {
        Version(text.to_string())
    }
}
