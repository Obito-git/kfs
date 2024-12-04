#![no_std]
#![allow(internal_features)]
#![feature(ptr_internals)]

mod vga;

use core::panic::PanicInfo;
use crate::vga::Writer;

const HEADER: &'static str = r"
____________/\\\_______/\\\\\\\\\_____
 __________/\\\\\_____/\\\///////\\\___
  ________/\\\/\\\____\///______\//\\\__
   ______/\\\/\/\\\______________/\\\/___
    ____/\\\/__\/\\\___________/\\\//_____
     __/\\\\\\\\\\\\\\\\_____/\\\//________
      _\///////////\\\//____/\\\/___________
       ___________\/\\\_____/\\\\\\\\\\\\\\\_
        ___________\///_____\///////////////__
";

#[no_mangle]
pub extern fn _start() -> ! {
    Writer::clear_screen();
    println!("{}", HEADER);
    loop {}
    Writer::clear_screen();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic: {}", info);
    loop {}
}