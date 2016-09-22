#![feature(lang_items, const_fn, unique)]
#![no_std]

extern crate rlibc;
extern crate spin;

// If we're building for x86_64
#[cfg(target_arch="x86_64")] #[path="arch/x86_64/mod.rs"] #[macro_use]
mod arch;

#[no_mangle]
pub extern "C" fn rust_main() {    
	print!("Hello, World!");
    loop {}
}

#[lang = "eh_personality"] extern "C" fn eh_personality() {}
#[lang = "panic_fmt"] extern "C" fn panic_fmt() -> ! {loop{}}


#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
