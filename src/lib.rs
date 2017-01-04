//! Hexdump: Generate a hexdump of data
//!
//! This crate provides a hex dumping utility.  A hexdump is a debugging representation of a block
//! of binary data.  It is intended to produce similar results to the `hexdump` linux utility.
//!
//! ```bash
//! $ echo 'This is a test message' | hexdump -C
//! 00000000  54 68 69 73 20 69 73 20  61 20 74 65 73 74 20 6d  |This is a test m|
//! 00000010  65 73 73 61 67 65 0a                              |essage.|
//! 00000017
//! ```
//!
//! But, instead of printing file data, it prints the contents of memory.  It is primarily used for
//! debugging.

use std::io::{self, Write};

/// A hex dumper.  This maintains the state, and progress of the hex dump.
struct Dumper<'a> {
    hex: String,
    ascii: String,
    count: usize,
    total_count: usize,
    write: &'a mut Write,
}

impl<'a> Dumper<'a> {
    fn new(write: &mut Write) -> Dumper {
        Dumper {
            hex: String::with_capacity(49),
            ascii: String::with_capacity(16),
            count: 0,
            total_count: 0,
            write: write,
        }
    }

    fn add_byte(&mut self, ch: u8) {
        if self.count == 16 {
            self.ship();
        }
        if self.count == 8 {
            self.hex.push(' ');
        }
        self.hex.push_str(&format!(" {:02x}", ch)[..]);
        self.ascii.push(if ch >= ' ' as u8 && ch <= '~' as u8 {
            ch as char
        } else {
            '.'
        });
        self.count += 1;
    }

    fn ship(&mut self) {
        if self.count == 0 {
            return;
        }

        writeln!(self.write, "{:06x} {:-49} |{}|", self.total_count, self.hex, self.ascii).unwrap();

        self.hex.clear();
        self.ascii.clear();
        self.total_count += self.count;
        self.count = 0;
    }
}

/// Print a simple block of '&[u8]` data to the given `Write`.
pub fn dump_to(data: &[u8], write: &mut Write) {
    let mut dump = Dumper::new(write);
    for ch in data {
        dump.add_byte(*ch);
    }
    dump.ship();
}

/// Print a simple block of `&[u8]` data.
pub fn dump_bytes(data: &[u8]) {
    let mut out = io::stdout();
    dump_to(data, &mut out);
}

#[test]
fn test_dump() {
    dump_bytes(b"This is a test message.");
}
