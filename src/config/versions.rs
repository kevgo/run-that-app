use super::Version;

/// a collection of Version instances
#[derive(Debug, PartialEq)]
pub struct Versions(Vec<Version>);

impl Versions {
    pub fn iter(&self) -> std::slice::Iter<'_, Version> {
        self.0.iter()
    }

    pub fn join(&self, sep: &str) -> String {
        let strings: Vec<&str> = self.0.iter().map(Version::as_str).collect();
        strings.join(sep)
    }

    /// provides the largest non-system version contained in this collection
    fn largest_non_system(&self) -> Option<&Version> {
        let mut result = None;
        for version in &self.0 {
            if version.is_system() {
                continue;
            }
            match result {
                Some(max) if version > max => result = Some(version),
                Some(_) => {}
                None => result = Some(version),
            }
        }
        result
    }

    pub fn push(&mut self, value: Version) {
        self.0.push(value);
    }

    /// Updates the largest non-system version in this collection with the given value.
    /// Returns the value that was replaced.
    pub fn update_largest_with(&mut self, value: &Version) -> Option<Version> {
        let Some(largest) = self.largest_non_system() else {
            return None;
        };
        if largest == value {
            return None;
        }
        let largest = largest.clone();
        let mut updated = None;
        for i in 0..self.0.len() {
            if self.0[i] == largest {
                updated = Some(self.0[i].clone());
                self.0[i] = value.clone();
            }
        }
        updated
    }
}

impl From<Version> for Versions {
    fn from(version: Version) -> Self {
        Versions(vec![version])
    }
}

impl From<&str> for Versions {
    fn from(version: &str) -> Self {
        Versions(vec![Version::from(version)])
    }
}

impl From<Vec<&str>> for Versions {
    fn from(versions: Vec<&str>) -> Self {
        let versions = versions.into_iter().map(Version::from).collect();
        Versions(versions)
    }
}
