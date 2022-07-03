// #![no_std]
#![feature(never_type)]
use std::io::IoSlice;
use std::arch::asm;
const BUFSIZ: usize = 8192*4;

fn main() -> ! {
    let buf = "n\n".repeat(BUFSIZ/2).into_bytes();
    let iov = &[IoSlice::new(&buf)];
    let ptr = iov.as_ptr();
    unsafe {
        loop {
            print!("");
            asm!("syscall",
                 in("rax") 278,
                 in("rdi") 1,
                 in("rsi") ptr,
                 in("rdx") 1,
                 in("r10") 2,                 
            );            
        }
    }    
}

