#![feature(lang_items)]
#![no_std]

extern crate rlibc;

// If we're building for x86_64
#[cfg(target_arch="x86_64")] #[path="arch/x86_64/mod.rs"]
mod arch;

#[no_mangle]
pub extern "C" fn rust_main() {    
    for (i, byte) in "Wow, look at this amazing text.".chars().enumerate() {
        arch::print_char(byte as u8, i);
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
