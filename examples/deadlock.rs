use std::io::{self, Write};

use defmt::write;
use defmt_stdout as _;

struct ReentrantObject(u8);

impl defmt::Format for ReentrantObject {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::info!("This is a deadlock?");
        write!(fmt, "ReentrantObject({})", self.0);
    }
}

fn main() {
    defmt::info!("This: {}", ReentrantObject(5));
    defmt::info!("Message");
    defmt::info!("And that: {}", ReentrantObject(8));
    let mut stdout = io::stdout().lock();
    stdout.write_all(b"hello world").unwrap();
    defmt::info!("{}", ReentrantObject(10));
    defmt::info!("Message");
    println!("Still holding the lock");
}
