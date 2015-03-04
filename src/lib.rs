#![deny(warnings)]
#![feature(fs)]
#![feature(io)]
#![feature(path)]

pub mod led;

use std::fs::File;
use std::io::{Read, Write, self};
use std::path::{AsPath, Path};

fn read<P: ?Sized>(path: &P, string: &mut String) -> io::Result<()> where P: AsPath {
    fn read_(path: &Path, string: &mut String) -> io::Result<()> {
        let mut file = try!(File::open(path));
        file.read_to_string(string)
    }

    read_(path.as_path(), string)
}

fn write<P: ?Sized>(path: &P, this: &str) -> io::Result<()> where P: AsPath {
    fn write_(path: &Path, this: &str) -> io::Result<()> {
        let mut file = try!(File::create(path));
        file.write_all(this.as_bytes())
    }

    write_(path.as_path(), this)
}
