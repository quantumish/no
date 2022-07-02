#![feature(never_type)]
use std::io::{self, Write, Error};
//const BUFSIZ: usize = 8192*7;
const BUFSIZ: usize = 8192*1;

fn main() -> Result<!, Error> {
    let buf = "n\n".repeat(BUFSIZ/2).into_bytes();
    let mut stdout = io::stdout().lock();
    //let mut out = io::stdout().lock();
    let mut out = io::BufWriter::new(stdout);
    loop {out.write(&buf)?;};
}
