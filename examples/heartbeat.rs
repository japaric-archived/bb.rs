extern crate bb;

use bb::led::{Heartbeat, Led, Zero};

fn main() {
    Led::new(Zero).set_trigger(Heartbeat);
}
