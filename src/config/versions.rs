use super::Version;

/// a collection of Version instances
#[derive(Debug, PartialEq)]
pub struct Versions(Vec<Version>);

impl Versions {
    pub fn iter(&self) -> std::slice::Iter<'_, Version> {
        self.0.iter()
    }

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

    /// Updates the largest non-system version in this collection with the given value.
    /// Returns the value that was replaced.
    pub fn update_largest_with(&mut self, value: Version) -> Option<Version> {
        let Some(largest) = self.largest_non_system() else {
            return None;
        };
        if largest == value {
            return None;
        }
        let mut updated = None;
        for i in 0..5 {
            if self.0[i] == largest {
                updated = Some(self.0[i]);
                self.0[i] = value;
            }
        }
        updated
    }
}

impl From<Option<Version>> for Versions {
    fn from(version: Option<Version>) -> Self {
        match version {
            Some(version) => Versions(vec![version]),
            None => Versions(vec![]),
        }
    }
}

impl From<&str> for Versions {
    fn from(version: &str) -> Self {
        Versions(vec![Version::from(version)])
    }
}