//! GPIO wired to the LEDs of the Beaglebone

use std::io;
use std::path::PathBuf;

/// Root folder (modulo number) that contains the files that control the GPIO
static ROOT: &'static str = "/sys/class/leds/beaglebone:green:usr";

/// LED identifier
#[derive(Copy)]
pub enum Number {
    /// First LED, by default is in heartbeat mode
    Zero = 0,
    /// Second LED, unused and powered off by default
    One = 1,
    /// Third LED, tracks CPU usage by default
    Three = 3,
    /// Fourth LED, tracks disk I/O by default
    Two = 2,
}

/// LED trigger modes
#[derive(Copy)]
pub enum Trigger {
    Heartbeat,
    None,
    Timer,
}

impl Trigger {
    fn from_str(s: &str) -> Trigger {
        match s {
            "heartbeat" => Trigger::Heartbeat,
            "none" => Trigger::None,
            "timer" => Trigger::Timer,
            _ => panic!("Unknown trigger mode: {}", s),
        }
    }

    fn to_str(&self) -> &'static str {
        match *self {
            Trigger::Heartbeat => "heartbeat",
            Trigger::None => "none",
            Trigger::Timer => "timer",
        }
    }
}

/// An LED controller
pub struct Led {
    root: PathBuf,
}

impl Led {
    /// Create access to an LED
    pub fn new(number: Number) -> Led {
        Led {
            root: PathBuf::new(&format!("{}{}", ROOT, number as u32)),
        }
    }

    /// Makes the LED blink
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use bb::led::{Led, Number};
    ///
    /// // On for one second, off for half a second
    /// Led::new(Number::Zero).blink(1000, 500);
    /// ```
    pub fn blink(&self, on_ms: u32, off_ms: u32) -> io::Result<()> {
        try!(self.set_trigger(Trigger::Timer));
        try!(::write(&self.root.join("delay_on"), &on_ms.to_string()));
        ::write(&self.root.join("delay_off"), &off_ms.to_string())
    }

    /// Changes the brightness of the LED
    // XXX Is `u8` enough?
    pub fn set_brightness(&self, brightness: u8) -> io::Result<()> {
        ::write(&self.root.join("brightness"), &*format!("{}", brightness))
    }

    /// Turns on the LED
    pub fn set_high(&self) -> io::Result<()> {
        try!(self.set_trigger(Trigger::None));
        self.set_brightness(1)
    }

    /// Turns off the LED
    pub fn set_low(&self) -> io::Result<()> {
        try!(self.set_trigger(Trigger::None));
        self.set_brightness(0)
    }

    /// Changes the trigger mode of the LED
    ///
    /// # Example
    ///
    /// ``` ignore
    /// use bb::led::{Led, Number, Trigger};
    ///
    /// Led::new(Number::Zero).set_trigger(Trigger::Heartbeat);
    /// println!("I'm alive!");
    /// ```
    pub fn set_trigger(&self, trigger: Trigger) -> io::Result<()> {
        ::write(&self.root.join("trigger"), trigger.to_str())
    }

    /// Returns the current trigger mode the LED is using
    pub fn trigger(&self) -> io::Result<Trigger> {
        let mut string = String::new();

        try!(::read(&self.root.join("trigger"), &mut string));

        match (string.find('['), string.find(']')) {
            (Some(start), Some(end)) => {
                Ok(Trigger::from_str(&string[start+"[".len()..end-"]".len()]))
            },
            _ => panic!("Failed to parse: {}", string),
        }
    }
}
