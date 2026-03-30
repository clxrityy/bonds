pub use api;

pub static CRATE = env!("CARGO_PKG_NAME");

pub fn main() {
    println!("Crate name: {}", CRATE);
}
