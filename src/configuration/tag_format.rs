use crate::configuration::Version;

/// how the Git tag describing the version of an application is formatted
pub(crate) enum TagFormat {
  /// version tags have no prefix or suffix, i.e. version 1.2.3 has tag "1.2.3"
  Plain,
  /// version tags have the prefix "v", i.e. version 1.2.3 has tag "v1.2.3"
  PrefixV,
  /// version tags have the given prefix
  Prefix(&'static str),
}

impl TagFormat {
  /// parses the given tag value into a Version
  pub(crate) fn parse<AS: AsRef<str>>(&self, value: AS) -> Version {
    match self {
      TagFormat::Plain => Version::from(value.as_ref()),
      TagFormat::PrefixV => Version::from(value.as_ref().strip_prefix("v").unwrap_or(value.as_ref())),
      TagFormat::Prefix(prefix) => Version::from(value.as_ref().strip_prefix(prefix).unwrap_or(value.as_ref())),
    }
  }

  pub(crate) fn format_version(&self, version: &Version) -> String {
    match self {
      TagFormat::Plain => version.to_string(),
      TagFormat::PrefixV => format!("v{version}"),
      TagFormat::Prefix(prefix) => format!("{prefix}{version}"),
    }
  }
}

#[cfg(test)]
mod tests {

  mod parse {
    use crate::configuration::{TagFormat, Version};

    #[test]
    fn plain() {
      assert_eq!(TagFormat::Plain.parse("1.2.3"), Version::from("1.2.3"));
    }

    #[test]
    fn prefix_v_with_v() {
      assert_eq!(TagFormat::PrefixV.parse("v1.0.0"), Version::from("1.0.0"));
    }

    #[test]
    fn prefix_v_without_v() {
      assert_eq!(TagFormat::PrefixV.parse("1.0.0"), Version::from("1.0.0"));
    }

    #[test]
    fn custom_matching_prefix() {
      assert_eq!(TagFormat::Prefix("bun-v").parse("bun-v1.2.3"), Version::from("1.2.3"));
    }

    #[test]
    fn custom_no_prefix() {
      assert_eq!(TagFormat::Prefix("bun-v").parse("1.2.3"), Version::from("1.2.3"));
    }
  }

  mod format_version {
    use crate::configuration::{TagFormat, Version};

    #[test]
    fn plain() {
      assert_eq!(TagFormat::Plain.format_version(&Version::from("4.5.6")), "4.5.6");
    }

    #[test]
    fn prefix_v() {
      assert_eq!(TagFormat::PrefixV.format_version(&Version::from("2.3.4")), "v2.3.4");
    }

    #[test]
    fn custom() {
      assert_eq!(TagFormat::Prefix("@pkg/").format_version(&Version::from("1.0.0")), "@pkg/1.0.0");
    }
  }
}
