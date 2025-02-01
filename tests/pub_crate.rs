use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{fs, io};

#[test]
fn test_add() -> std::io::Result<()> {
  let current_dir = std::env::current_dir()?;
  let mut files = Vec::new();
  find_files(&current_dir, &mut files)?;
  let mut exit_code = 0;
  for file in files {
    for (index, line) in lines_in_file(&file)?.enumerate() {
      if let Ok(line_content) = line {
        if line_content.trim_start().starts_with("pub ") {
          println!("{}:{} {}", file.to_string_lossy(), index + 1, line_content);
          exit_code = 1;
        }
      }
    }
  }
  std::process::exit(exit_code);
}

fn find_files(dir: &Path, result: &mut Vec<std::path::PathBuf>) -> io::Result<()> {
  if !dir.is_dir() {
    return Ok(());
  }
  for entry in fs::read_dir(dir)? {
    let path = entry?.path();
    if path.ends_with("target") || path.ends_with("cargo_task_util") {
      continue;
    }
    if path.is_dir() {
      find_files(&path, result)?;
    } else if let Some(extension) = path.extension() {
      if extension == "rs" {
        result.push(path);
      }
    }
  }
  Ok(())
}

fn lines_in_file(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
