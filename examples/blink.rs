extern crate bb;

use std::process;

use bb::led::{Led, Number};

fn main() {
    Led::new(Number::Zero).blink(500, 500).unwrap_or_else(|e| {
        println!("error: {}", e);
        process::exit(1)
    })
}
