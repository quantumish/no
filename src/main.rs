#![feature(never_type)]
use std::io::{self, IoSlice};
use std::os::unix::io::{AsRawFd};
use nix::fcntl::{vmsplice, SpliceFFlags};
const BUFSIZ: usize = 8192*4;

fn main() -> ! {
    let buf = "n\n".repeat(BUFSIZ/2).into_bytes();
    let stdout = io::stdout().lock().as_raw_fd();
    loop {vmsplice(stdout, &[IoSlice::new(&buf)], SpliceFFlags::SPLICE_F_NONBLOCK);}
}
