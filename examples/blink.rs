extern crate bb;

use bb::led::{Led, Zero};

fn main() {
    Led::new(Zero).blink(500, 500);
}
