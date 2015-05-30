extern crate bb;

use std::process;

use bb::led::{Led, Number, Trigger};

fn main() {
    Led::new(Number::Zero).set(Trigger::Heartbeat).unwrap_or_else(|e| {
        println!("error: {}", e);
        process::exit(1)
    })
}
