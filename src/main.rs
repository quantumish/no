// #![no_std]
#![feature(never_type)]
use std::io::IoSlice;
use nix::fcntl::{FcntlArg, fcntl};
const BUFSIZ: usize = 1024*1024;

fn main() -> ! {
    fcntl(1, FcntlArg::F_SETPIPE_SZ(1024*1024));
    let buf = "n\n".repeat(BUFSIZ/2).into_bytes();    
    let iovs = &[IoSlice::new(&buf); 1];
    let ptr = iovs.as_ptr();    
    unsafe {
        loop {
            core::arch::asm!(
                "syscall",
                in("rax") 278,
                in("rdi") 1,
                in("rsi") ptr,
                in("rdx") 1,
                in("r10") 2
            );
            print!("")
        }
    }
}





