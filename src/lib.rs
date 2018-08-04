#[macro_use]
mod macros;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn hello() -> String {
    return format!("Hello, cosworth v{}!", VERSION);
}
