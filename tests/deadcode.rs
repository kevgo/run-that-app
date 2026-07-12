//! End-to-end test that runs warnalyzer (<https://github.com/est31/warnalyzer>) with its
//! scip backend against this codebase, to catch dead code that isn't reachable from any
//! entry point.
//!
//! warnalyzer produces two well-known categories of false positives that this test filters
//! out before checking for regressions:
//!
//! - findings located inside `#[cfg(test)]` items, since those aren't dead, they are only
//!   used by `cargo test`
//! - findings for methods that are only ever called through dynamic dispatch (`dyn Trait`)
//!   or compiler/macro magic (`Display`, `From`, `IntoIterator`, ...), which warnalyzer's
//!   scip backend cannot trace back to a caller
//!   (see <https://github.com/est31/warnalyzer#false-positives>)
//!
//! Anything that doesn't fall into one of those categories has to be listed explicitly in
//! `KNOWN_FINDINGS` below, so that this test fails as soon as *new* dead code shows up.

#![allow(clippy::expect_used)]

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// traits whose methods are invoked through mechanisms that warnalyzer cannot see:
/// dynamic dispatch (`AppDefinition`, `Archive`) or language/macro magic (the rest)
const DYNAMIC_TRAITS: &[&str] = &[
  "AppDefinition",
  "Archive",
  "Display",
  "Debug",
  "PartialEq",
  "PartialOrd",
  "AsRef",
  "From",
  "IntoIterator",
  "Iterator",
];

/// findings that are real but not false positives, listed here as `(file, item name)` so that
/// this test still fails on any *other, new* dead code
const KNOWN_FINDINGS: &[(&str, &str)] = &[
  // entry points are always reported as unused by warnalyzer, see the link in the module docs above
  ("src/main.rs", "main"),
  // unused public API, kept around for now
  ("src/applications/mod.rs", "iter"),
];

struct Finding {
  file: String,
  line: usize,
  name: String,
  raw: String,
}

#[test]
fn no_unexpected_dead_code() {
  let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let output = Command::new("warnalyzer")
    .arg(&repo_root)
    .output()
    .expect("could not run warnalyzer, install it via `make setup` or see https://github.com/est31/warnalyzer");
  let stdout = String::from_utf8_lossy(&output.stdout);
  let findings = parse_findings(&stdout);

  let mut cfg_test_ranges_by_file: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
  let mut dynamic_impl_ranges_by_file: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

  let unexpected: Vec<&str> = findings
    .iter()
    .filter(|finding| {
      let cfg_test_ranges = cfg_test_ranges_by_file
        .entry(finding.file.clone())
        .or_insert_with(|| cfg_test_ranges(&repo_root.join(&finding.file)));
      if in_ranges(finding.line, cfg_test_ranges) {
        return false;
      }
      let dynamic_impl_ranges = dynamic_impl_ranges_by_file
        .entry(finding.file.clone())
        .or_insert_with(|| dynamic_trait_impl_ranges(&repo_root.join(&finding.file)));
      if in_ranges(finding.line, dynamic_impl_ranges) {
        return false;
      }
      !KNOWN_FINDINGS.contains(&(finding.file.as_str(), finding.name.as_str()))
    })
    .map(|finding| finding.raw.as_str())
    .collect();

  assert!(
    unexpected.is_empty(),
    "warnalyzer found dead code that isn't accounted for.\n\
     Either remove the dead code, or (if it's a false positive) add it to KNOWN_FINDINGS in tests/deadcode.rs:\n\n{}",
    unexpected.join("\n")
  );
}

/// parses lines like `src/applications/go.rs:87:6: unused Method 'allowed_versions'`
fn parse_findings(output: &str) -> Vec<Finding> {
  output
    .lines()
    .filter_map(|line| {
      let (location, rest) = line.split_once(": unused ")?;
      let mut parts = location.splitn(3, ':');
      let file = parts.next()?.to_string();
      let line_number = parts.next()?.parse().ok()?;
      let name = rest.split('\'').nth(1)?.to_string();
      Some(Finding {
        file,
        line: line_number,
        name,
        raw: line.to_string(),
      })
    })
    .collect()
}

fn in_ranges(line: usize, ranges: &[(usize, usize)]) -> bool {
  ranges.iter().any(|&(start, end)| line >= start && line <= end)
}

/// line ranges (1-based, inclusive) of all `#[cfg(test)]` items and `#[test]` functions in the
/// given file. The latter covers `#[test]` functions in integration test files under `tests/`,
/// which have no `#[cfg(test)]` attribute of their own (the whole file is test-only already)
/// but are still only ever invoked by the test harness, never by other code.
fn cfg_test_ranges(path: &Path) -> Vec<(usize, usize)> {
  let Ok(content) = fs::read_to_string(path) else {
    return Vec::new();
  };
  let lines: Vec<&str> = content.lines().collect();
  let mut ranges = Vec::new();
  for (i, line) in lines.iter().enumerate() {
    if !(line.contains("#[cfg(test)]") || line.trim() == "#[test]") {
      continue;
    }
    let mut item_start = i + 1;
    while item_start < lines.len() && (lines[item_start].trim().is_empty() || lines[item_start].trim_start().starts_with("#[")) {
      item_start += 1;
    }
    if let Some(end) = block_end(&lines, item_start) {
      ranges.push((i + 1, end + 1));
    }
  }
  ranges
}

/// line ranges (1-based, inclusive) of all `impl <Trait> for ...` blocks in the given file
/// whose trait is listed in [`DYNAMIC_TRAITS`]
fn dynamic_trait_impl_ranges(path: &Path) -> Vec<(usize, usize)> {
  let Ok(content) = fs::read_to_string(path) else {
    return Vec::new();
  };
  let lines: Vec<&str> = content.lines().collect();
  let mut ranges = Vec::new();
  for (i, line) in lines.iter().enumerate() {
    let Some(trait_name) = impl_trait_name(line) else {
      continue;
    };
    if !DYNAMIC_TRAITS.contains(&trait_name) {
      continue;
    }
    if let Some(end) = block_end(&lines, i) {
      ranges.push((i + 1, end + 1));
    }
  }
  ranges
}

/// extracts the trait name from an `impl <generics> TraitName<generics> for Type` header line,
/// e.g. `impl<'a> IntoIterator for &'a Apps {` -> `Some("IntoIterator")`.
/// returns `None` for inherent impls (`impl Type {`), which have no `for` clause.
fn impl_trait_name(line: &str) -> Option<&str> {
  let rest = line.trim_start().strip_prefix("impl")?.trim_start();
  let rest = match rest.strip_prefix('<') {
    Some(after_generics) => after_generics.split_once('>')?.1.trim_start(),
    None => rest,
  };
  let (trait_part, _type_part) = rest.split_once(" for ")?;
  let trait_name = trait_part.split(['<', ' ']).next()?.rsplit("::").next()?;
  if trait_name.is_empty() { None } else { Some(trait_name) }
}

/// finds the line (0-based index into `lines`) on which the block starting at or after `from`
/// ends, i.e. the line containing the semicolon of a bodyless item, or the closing brace that
/// balances the first opening brace found from `from` onwards
fn block_end(lines: &[&str], from: usize) -> Option<usize> {
  for start in from..lines.len() {
    if lines[start].contains('{') {
      let mut depth = 0i32;
      for (i, line) in lines.iter().enumerate().skip(start) {
        depth += i32::try_from(line.matches('{').count()).ok()? - i32::try_from(line.matches('}').count()).ok()?;
        if depth == 0 {
          return Some(i);
        }
      }
      return None;
    }
    if lines[start].contains(';') {
      return Some(start);
    }
  }
  None
}
