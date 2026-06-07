//! Custom linters for this codebase.

use std::collections::HashSet;
use std::fs;

const APPLICATIONS_MOD: &str = "src/applications/mod.rs";

#[test]
fn applications_mod_has_pub_use_for_every_module() {
  let content = fs::read_to_string(APPLICATIONS_MOD).expect("read applications mod.rs");
  let modules = application_modules(&content);
  let pub_uses = application_pub_use_modules(&content);

  let missing: Vec<&str> = modules.iter().filter(|module| !pub_uses.contains(*module)).map(String::as_str).collect();

  assert!(
    missing.is_empty(),
    "src/applications/mod.rs is missing `pub use` re-exports for: {}\n\
     For each `mod xxx;` declaration, add a line like `pub use xxx::Xxx;`.",
    missing.join(", ")
  );
}

fn application_modules(content: &str) -> Vec<String> {
  let mut modules = Vec::new();

  for line in content.lines() {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with("//!") {
      continue;
    }
    if let Some(name) = trimmed.strip_prefix("mod ").and_then(|rest| rest.strip_suffix(';')) {
      modules.push(name.to_string());
      continue;
    }
    break;
  }

  modules
}

fn application_pub_use_modules(content: &str) -> HashSet<String> {
  content
    .lines()
    .filter_map(|line| {
      let trimmed = line.trim();
      let rest = trimmed.strip_prefix("pub use ")?;
      let module = rest.split("::").next()?;
      Some(module.to_string())
    })
    .collect()
}
