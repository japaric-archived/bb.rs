extern crate bb;

use bb::led::{Led, Number};

fn main() {
    Led::new(Number::Zero).blink(500, 500);
}
