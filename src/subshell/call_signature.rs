use super::Executable;

pub fn call_signature(executable: &Executable, args: &[String]) -> String {
    format!("{} {}", executable.0.to_string_lossy(), args.join(" "))
}
