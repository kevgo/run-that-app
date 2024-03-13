/// helper macro that prints the given text to STDERR and flushes the output
#[macro_export]
macro_rules! fprint {
    ($($arg:tt)*) => {{
        eprint!($($arg)*);
        let _ = io::stderr().flush();
    }};
}
