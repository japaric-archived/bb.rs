//! Operations that "just" `fail!` if they don't succeed

use std::io::File;

pub fn read(path: &Path) -> String {
    let path_ = path.display();

    match File::open(path) {
        Err(e) => fail!("Couldn't open {} ({})", path_, e),
        Ok(mut f) => match f.read_to_string() {
            Err(e) => fail!("Couldn't read {} ({})", path_, e),
            Ok(s) => s,
        }
    }
}

pub fn write(path: &Path, s: &str) {
    let path_ = path.display();

    match File::create(path) {
        Err(e) => fail!("Couldn't create {} ({})", path_, e),
        Ok(mut f) => if let Err(e) = f.write_str(s) {
            fail!("Couldn't write to {} ({})", path_, e);
        }
    }
}
