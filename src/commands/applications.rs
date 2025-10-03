use crate::applications::Apps;
use std::process::ExitCode;

pub(crate) fn long(apps: Apps) -> ExitCode {
  let width = apps.longest_name_length() + 1;
  for app in apps {
    println!("{:<width$} {}", app.name(), app.homepage());
  }
  ExitCode::SUCCESS
}

pub(crate) fn short(apps: Apps) -> ExitCode {
  for app in apps {
    println!("{}", app.name());
  }
  ExitCode::SUCCESS
}
