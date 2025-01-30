use std::fmt::Display;

pub(crate) fn exit(err: impl Display) -> ! {
  println!("ERROR: {err}");
  std::process::exit(1);
}
