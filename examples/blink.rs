#![feature(exit_status)]

extern crate bb;

use std::env;

use bb::led::{Led, Number};

fn main() {
    if let Err(e) = Led::new(Number::Zero).blink(500, 500) {
        println!("{}", e);
        env::set_exit_status(1);
    }
}
