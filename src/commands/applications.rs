use crate::applications;
use std::process::ExitCode;

pub(crate) fn long() -> ExitCode {
  let apps = applications::all();
  let width = apps.longest_name_length() + 1;
  for app in apps.iter() {
    println!("{:<width$} {}", app.name(), app.homepage());
  }
  ExitCode::SUCCESS
}

pub(crate) fn short() -> ExitCode {
  let apps = applications::all();
  for app in apps.iter() {
    println!("{}", app.name());
  }
  ExitCode::SUCCESS
}
