//! GPIO wired to the LEDs of the Beaglebone

use std::fs::File;
use std::io::{Read, Write, self};
use std::path::PathBuf;

/// LED identifier
#[derive(Clone, Copy)]
pub enum Number {
    /// First LED, by default is in heartbeat mode
    Zero,
    /// Second LED, unused and powered off by default
    One,
    /// Third LED, tracks CPU usage by default
    Three,
    /// Fourth LED, tracks disk I/O by default
    Two,
}

impl Number {
    fn to_str(&self) -> &'static str {
        match *self {
            Number::Zero => "0",
            Number::One => "1",
            Number::Two => "2",
            Number::Three => "3",
        }
    }
}

/// LED trigger modes
#[derive(Clone, Copy)]
pub enum Trigger {
    Heartbeat,
    None,
    Timer,
}

impl Trigger {
    fn from_str(s: &str) -> Option<Trigger> {
        match s {
            "heartbeat" => Some(Trigger::Heartbeat),
            "none" => Some(Trigger::None),
            "timer" => Some(Trigger::Timer),
            _ => None,
        }
    }

    fn to_bytes(&self) -> &'static [u8] {
        match *self {
            Trigger::Heartbeat => b"heartbeat",
            Trigger::None => b"none",
            Trigger::Timer => b"timer",
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
        // Root folder (modulo number) that contains the files that control the GPIO
        const ROOT: &'static str = "/sys/class/leds/beaglebone:green:usr";

        Led {
            root: PathBuf::from(format!("{}{}", ROOT, number.to_str())),
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
        try!(self.set(Trigger::Timer));
        try!(try!(File::create(self.root.join("delay_on"))).write_all(on_ms.to_string().as_bytes()));
        try!(File::create(self.root.join("delay_off"))).write_all(off_ms.to_string().as_bytes())
    }

    /// Changes the brightness of the LED
    pub fn set_brightness(&self, brightness: u32) -> io::Result<()> {
        try!(File::create(self.root.join("brightness"))).write_all(brightness.to_string().as_bytes())
    }

    /// Turns on the LED
    pub fn set_high(&self) -> io::Result<()> {
        try!(self.set(Trigger::None));
        self.set_brightness(1)
    }

    /// Turns off the LED
    pub fn set_low(&self) -> io::Result<()> {
        try!(self.set(Trigger::None));
        self.set_brightness(0)
    }

    /// Changes the trigger mode of the LED
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use bb::led::{Led, Number, Trigger};
    ///
    /// Led::new(Number::Zero).set(Trigger::Heartbeat);
    /// println!("I'm alive!");
    /// ```
    pub fn set(&self, trigger: Trigger) -> io::Result<()> {
        try!(File::create(self.root.join("trigger"))).write_all(trigger.to_bytes())
    }

    /// Returns the current trigger mode the LED is using
    pub fn trigger(&self) -> io::Result<Trigger> {
        let mut string = String::with_capacity(128);

        try!(try!(File::open(self.root.join("trigger"))).read_to_string(&mut string));

        match string.split('[').skip(1).next().and_then(|s| s.split(']').next()) {
            Some(s) => match Trigger::from_str(s) {
                Some(trigger) => Ok(trigger),
                None => panic!("Unknown trigger mode: {}", s),
            },
            None => panic!("Failed to parse: {}", string),
        }
    }
}
