use crate::prelude::*;
use std::cmp::Ordering;
use std::fmt::Display;
use std::path::Path;

/// the desired version of an application
#[derive(Clone, Debug, PartialEq)]
pub struct Version(String);

impl PartialOrd for Version {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    let result = compare_semver(self, other);
    if result.is_some() {
      return result;
    }
    // no conclusive semver order --> compare alphabetically
    self.as_str().partial_cmp(other.as_str())
  }
}

impl Version {
  pub(crate) fn as_str(&self) -> &str {
    &self.0
  }

  pub(crate) fn semver(&self) -> Result<semver::Version> {
    semver::Version::parse(&self.0).map_err(|err| UserError::CannotParseSemverVersion {
      expression: self.0.to_string(),
      reason: err.to_string(),
    })
  }
}

impl AsRef<Path> for Version {
  fn as_ref(&self) -> &Path {
    let text: &str = self.as_str();
    Path::new(text)
  }
}

impl Display for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl From<&str> for Version {
  fn from(text: &str) -> Self {
    Version(text.to_string())
  }
}

impl From<String> for Version {
  fn from(text: String) -> Self {
    Version(text)
  }
}

impl PartialEq<str> for Version {
  fn eq(&self, other: &str) -> bool {
    self.as_str() == other
  }
}

impl PartialEq<String> for Version {
  fn eq(&self, other: &String) -> bool {
    self.as_str() == *other
  }
}

fn compare_semver(v1: &Version, v2: &Version) -> Option<Ordering> {
  let Ok(self_version) = semver::Version::parse(v1.as_str()) else {
    return None;
  };
  let Ok(other_version) = semver::Version::parse(v2.as_str()) else {
    return None;
  };
  self_version.partial_cmp(&other_version)
}

#[cfg(test)]
mod tests {

  mod partial_cmp {
    use crate::configuration::Version;

    #[test]
    fn semantic() {
      let bigger = Version::from("3.10.2");
      let smaller = Version::from("3.2.1");
      assert!(bigger > smaller);
    }

    #[test]
    fn tag() {
      let pre_release = Version::from("1.2.3-alpha");
      let final_release = Version::from("1.2.3");
      assert!(pre_release < final_release);
    }
  }
}
