#![feature(lang_items, const_fn, unique, asm)]
#![no_std]

extern crate rlibc;

// If we're building for x86_64
#[cfg(target_arch="x86_64")] #[path="arch/x86_64/mod.rs"]
mod arch;

use core::ptr::Unique;
use arch::vga::*;

#[no_mangle]
pub extern "C" fn rust_main() {    
    let color = ColorCode::new(Color::Green, Color::Black);
    let buffer = unsafe { Unique::new(0xb8000 as *mut _) };
    let mut vga = VGA::new(buffer, color);
        
    for byte in "Hello, World!".chars()
    {
        vga.write_char(byte as u8);
    }
	loop {}
}

#[lang = "eh_personality"] extern "C" fn eh_personality() {}
#[lang = "panic_fmt"] extern "C" fn panic_fmt() -> ! {loop{}}


#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
