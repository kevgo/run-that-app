use crate::apps;
use std::process::ExitCode;

pub fn apps() -> ExitCode {
  let apps = apps::all();
  let width = apps.longest_name_length() + 1;
  for app in apps.iter() {
    println!("{:<width$} {}", app.name().as_str(), app.homepage());
  }
  ExitCode::SUCCESS
}
