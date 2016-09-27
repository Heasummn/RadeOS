#![feature(lang_items, const_fn, unique)]
#![no_std]

extern crate rlibc;
extern crate spin;

// If we're building for x86_64
#[cfg(target_arch="x86_64")] #[path="arch/x86_64/mod.rs"] #[macro_use]
mod arch;

#[macro_use]
mod logging;

#[macro_use]
mod multiboot;

#[no_mangle]
pub extern "C" fn rust_main() {
    kprintln!("Booting!");
    
    arch::vga::init_vga();
    kinfo!("Initialized VGA");

    kerror!("Oh no, something went wrong! (Not really, we're doing this as a test).");
}

#[lang = "eh_personality"] extern "C" fn eh_personality() {}
#[lang = "panic_fmt"] extern "C" fn panic_fmt() -> ! {loop{}}


#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
