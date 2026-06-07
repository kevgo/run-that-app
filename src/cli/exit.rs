use std::fmt::Display;

pub fn exit(err: impl Display) -> ! {
  println!("ERROR: {err}");
  std::process::exit(1);
}
