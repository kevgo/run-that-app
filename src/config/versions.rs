use super::Version;

/// a collection of Version instances
pub struct Versions(Vec<Version>);

impl Versions {
    pub fn join(&self, sep: &str) -> String {
        let strings: Vec<&str> = self.0.iter().map(|version| version.as_str()).collect();
        strings.join(sep)
    }

    /// provides the largest non-system version contained in this collection
    fn largest_non_system(&self) -> Option<Version> {
        let mut result = None;
        for version in self.0 {
            if version.is_system() {
                continue;
            }
            match result {
                Some(max) if version > max => result = Some(version),
                Some(max) => {}
                None => result = Some(version),
            }
        }
        result
    }

    /// provides a new collection that has the same elements as this one except the largest version is replaced with the given value
    pub fn update_largest_with(self, value: Version) -> Versions {
        let largest = self.largest_non_system();
    }
}
