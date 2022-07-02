#![feature(never_type)]
use std::io::{self, Write, Error};
const BUFSIZ: usize = 8192*1;

fn main() -> Result<!, Error> {
    let buf = "n\n".repeat(BUFSIZ/2).into_bytes();
    let out = io::stdout();
    let mut stdout = out.lock();
    loop {stdout.write(&buf).unwrap(); stdout.flush();};
}
