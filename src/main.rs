#![feature(never_type)]
use std::io::{self, Write, Error};
const BUFSIZ: usize = 8192*7;

fn main() -> Result<!, Error> {
    let buf = "n\n".repeat(BUFSIZ/2).into_bytes();
    loop {io::stdout().write(&buf)?;};
}
