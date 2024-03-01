/// a string that represents
#[derive(Debug, Default, PartialEq)]
pub struct Version(Option<String>);

impl Version{
    pub fn none() -> Version {
        Version(None)
    }
}

impl From<&str> for Version {
    fn from(text: &str) -> Self {
        if text.is_empty() {
            return Version(None);
        }
        Version(Some(text.to_string()))
    }
}
