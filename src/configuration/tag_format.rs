use crate::configuration::Version;

/// how the Git tag describing the version of an application is formatted
pub(crate) enum TagFormat {
  /// version tags have no prefix or suffix, i.e. version 1.2.3 has tag "1.2.3"
  Plain,
  /// version tags have the prefix "v", i.e. version 1.2.3 has tag "v1.2.3"
  PrefixV,
  /// version tags have the given prefix
  Prefixed(&'static str),
}

impl TagFormat {
  /// parses the given tag value into a Version
  pub(crate) fn parse<AS: AsRef<str>>(&self, value: AS) -> Version {
    match self {
      TagFormat::Plain => Version::from(value.as_ref()),
      TagFormat::PrefixV => Version::from(value.as_ref().strip_prefix("v").unwrap_or(value.as_ref())),
      TagFormat::Prefixed(prefix) => Version::from(value.as_ref().strip_prefix(prefix).unwrap_or(value.as_ref())),
    }
  }

  pub(crate) fn format_version(&self, version: &Version) -> String {
    match self {
      TagFormat::Plain => version.to_string(),
      TagFormat::PrefixV => format!("v{version}"),
      TagFormat::Prefixed(prefix) => format!("{prefix}{version}"),
    }
  }
}
