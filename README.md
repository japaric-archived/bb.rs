# Status

This project is **DEPRECATED**. This was mostly an experiment of doing Rust development on a
BeagleBone. If you are looking for similar functionality, check [sysfs-gpio] which is more general
because it supports any device.

[sysfs-gpio]: https://crates.io/crates/sysfs_gpio

This project won't receive further updates or bug fixes.

-- @japaric, 2016/02/08

---

[![Build Status][status]](https://travis-ci.org/japaric/bb.rs)

# `bb.rs`

Rust library to do GPIO on the beaglebone

This library has been fully developed *on* the beaglebone using cargo!

(There are no official rust/cargo nightlies for ARM at the moment, but I'm
hosting my own [here][ruststrap])

# [Documentation][docs]

# Canonical example

``` rust
extern crate bb;

use bb::led::{Led, Zero};

fn main() {
    // Blink LED0: ON for 500 ms, OFF for 500 ms, repeat
    Led::new(Zero).blink(500, 500);
}
```

# Cargo this

```
# Cargo.toml
[dependencies.bb]
git = "https://github.com/japaric/bb.rs"
```

# Features

Very early WIP

* Only the GPIO wired to the board LEDs can be used
* Device tree stuff will come later

# License

bb.rs is dual licensed under the Apache 2.0 license and the MIT license.

See LICENSE-APACHE and LICENSE-MIT for more details.

[docs]: http://japaric.github.io/bb.rs/bb/
[ruststrap]: https://github.com/japaric/ruststrap
[status]: https://travis-ci.org/japaric/bb.rs.svg?branch=master
