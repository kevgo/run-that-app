pub fn exit(err: &str) -> ! {
  println!("ERROR: {err}");
  std::process::exit(1);
}
