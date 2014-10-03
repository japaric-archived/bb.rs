//! GPIO wired to the LEDs of the Beaglebone

use just;

/// Root folder (modulo number) that contains the files that control the GPIO
static ROOT: &'static str = "/sys/class/leds/beaglebone:green:usr";

/// LED identifier
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
pub enum Trigger {
    Heartbeat,
    NoTrigger,
    Timer,
}

impl Trigger {
    fn from_str(s: &str) -> Option<Trigger> {
        match s {
            "heartbeat" => Some(Heartbeat),
            "none" => Some(NoTrigger),
            "timer" => Some(Timer),
            _ => None,
        }
    }

    fn to_str(&self) -> &'static str {
        match *self {
            Heartbeat => "heartbeat",
            NoTrigger => "none",
            Timer => "timer",
        }
    }
}

/// An LED controller
pub struct Led {
    root: Path,
}

impl Led {
    /// Create access to an LED
    pub fn new(number: Number) -> Led {
        Led {
            root: Path::new(format!("{}{}", ROOT, number as u8)),
        }
    }

    /// Makes the LED blink
    ///
    /// # Example
    ///
    /// ``` ignore
    /// use bb::led::{Led, Zero};
    ///
    /// // On for one second, off for half a second
    /// Led::new(Zero).blink(1000, 500);
    /// ```
    // XXX Not sure about how big can `on_ms` and `off_ms` be
    pub fn blink(&self, on_ms: u16, off_ms: u16) {
        self.set_trigger(Timer);
        just::write(&self.root.join("delay_on"), format!("{}", on_ms)[]);
        just::write(&self.root.join("delay_off"), format!("{}", off_ms)[]);
    }

    /// Changes the brightness of the LED
    // XXX Is `u8` enough?
    pub fn set_brightness(&self, brightness: u8) {
        just::write(&self.root.join("brightness"), format!("{}", brightness)[])
    }

    /// Turns on the LED
    pub fn set_high(&self) {
        self.set_trigger(NoTrigger);
        self.set_brightness(1);
    }

    /// Turns off the LED
    pub fn set_low(&self) {
        self.set_trigger(NoTrigger);
        self.set_brightness(0);
    }

    /// Changes the trigger mode of the LED
    ///
    /// # Example
    ///
    /// ``` ignore
    /// use bb::led::{Led, Heartbeat, Zero};
    ///
    /// Led::new(Zero).set_trigger(Heartbeat);
    /// println!("I'm alive!");
    /// ```
    pub fn set_trigger(&self, trigger: Trigger) {
        just::write(&self.root.join("trigger"), trigger.to_str());
    }

    /// Returns the current trigger mode the LED is using
    pub fn trigger(&self) -> Trigger {
        let s = just::read(&self.root.join("trigger"));
        match s[].split('[').skip(1).next().and_then(|s| s.split(']').next()) {
            Some(s) => match Trigger::from_str(s) {
                Some(trigger) => trigger,
                None => fail!("Unknown trigger mode: {}", s),
            },
            None => fail!("Failed to parse: {}", s),
        }
    }
}
