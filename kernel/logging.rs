#[macro_extern]
macro_rules! kprint {
    ($($arg:tt)*) => 
        ({
            use core::fmt::Write;
            let mut b = $crate::arch::vga::BUFFER.lock();
            b.write_fmt(format_args!($($arg)*)).unwrap();        
        });
}

#[macro_extern]
macro_rules! kprintln {
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_extern]
macro_rules! kdebug {
    ($fmt:expr) => 
    ({
            // Convert text color to blue
        use $crate::arch::vga::{ColorCode, Color, DEF_COLOR};
        $crate::arch::vga::BUFFER.lock().set_color(ColorCode::new(Color::Blue, Color::Black));
            
        kprintln!(concat!("Debug: ", $fmt));
        // Convert it back
        $crate::arch::vga::BUFFER.lock().set_color(DEF_COLOR);
    });

    ($fmt:expr, $($arg:tt)*) => ({
        use $crate::arch::vga::{ColorCode, Color, DEF_COLOR};
        $crate::arch::vga::BUFFER.lock().set_color(ColorCode::new(Color::Blue, Color::Black));
            
        kprintln!(concat!("Debug: ", $fmt), $($arg)*);
        $crate::arch::vga::BUFFER.lock().set_color(DEF_COLOR);
    });
}

#[macro_extern]
macro_rules! kinfo {
    ($fmt:expr) => (kprintln!(concat!("Info: ", $fmt)));
    ($fmt:expr, $($arg:tt)*) => (kprintln!(concat!("Info: ", $fmt), $($arg)*));
}

#[macro_extern]
macro_rules! kerror {
    ($fmt:expr) =>
        ({
            // Convert color to red
            use $crate::arch::vga::{ColorCode, Color, DEF_COLOR};
            $crate::arch::vga::BUFFER.lock().set_color(ColorCode::new(Color::Red, Color::Black));

            kprintln!(concat!("Error: ", $fmt));
            // Convert it back
            $crate::arch::vga::BUFFER.lock().set_color(DEF_COLOR);
            loop {}
        });

    ($fmt:expr, $($arg:tt)*) => 
        ({
            use $crate::arch::vga::{ColorCode, Color, DEF_COLOR};
            $crate::arch::vga::BUFFER.lock().set_color(ColorCode::new(Color::Red, Color::Black));
            
            kprintln!(concat!("Error: ", $fmt), $($arg)*); 
            $crate::arch::vga::BUFFER.lock().set_color(DEF_COLOR);
            loop {}
        });
}
