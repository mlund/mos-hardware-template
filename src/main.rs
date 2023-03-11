//! 10print algorithm in Rust
//!
//! 8-bit show and tell has a great movie about this:
//! https://www.youtube.com/watch?v=IPP-EMBQPhE

#![no_std]
#![feature(start)]

use core::panic::PanicInfo;
use mos_hardware::{c64, sid};
use rand::seq::SliceRandom;
use ufmt_stdio::*;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    let mut rng = sid::SIDRng::new(c64::sid());
    for offset in 0..40 * 25 {
        let character = [77u8, 78u8].choose(&mut rng).copied().unwrap();
        unsafe {
            c64::DEFAULT_VIDEO_MEMORY
                .add(offset)
                .write_volatile(character)
        };
    }
    println!("HELLO C64 FROM RUST");
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
