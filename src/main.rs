#![feature(never_type)]
use std::io::{self, Write, Error};
const BUFSIZ: usize = 8192*7;

fn main() -> Result<!, Error> {
    let mut buf: [u8; BUFSIZ] = [110; BUFSIZ];
    for i in 0..BUFSIZ/2 {buf[i*2] = 10;}
    loop {io::stdout().write(&buf)?;};
}
