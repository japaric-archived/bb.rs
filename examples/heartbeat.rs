extern crate bb;

use bb::led::{Led, Number, Trigger};

fn main() {
    Led::new(Number::Zero).set_trigger(Trigger::Heartbeat);
}
