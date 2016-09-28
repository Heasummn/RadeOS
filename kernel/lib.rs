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

use multiboot::MultiBootInfo;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_address: usize) {
    kprintln!("Booting!");
    
    arch::vga::init_vga();
    kinfo!("Initialized VGA");

    kprintln!("Multiboot information is at {}", multiboot_address);
     
    let info = unsafe { &*(multiboot_address as *const MultiBootInfo) };
   
    // Accessing info causes a page fault.
    // It would really help if we have an interrupt handler. 
    // So we will work on that instead of a multiboot loader.
    
    kerror!("Oh no, something went wrong! (Not really, we're doing this as a test).");
    
}

#[lang = "eh_personality"] extern "C" fn eh_personality() {}
#[lang = "panic_fmt"] extern "C" fn panic_fmt() -> ! {loop{}}


#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
