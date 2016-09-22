use core::ptr::Unique;
use spin::Mutex;

const BUFFER_WIDTH: usize =  80;
const BUFFER_HEIGHT: usize =  25;

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1, 
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15
}

// A foreground and a background
#[derive(Clone, Copy, Debug)]
pub struct ColorCode(u8);
impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | foreground as u8)
    }
}

const DEF_COLOR: ColorCode = ColorCode::new(Color::LightGreen, Color::Black);

#[repr(C)]
struct VGAChar { 
    text: u8,
    color: ColorCode
}

pub struct Buffer {
    chars: [[VGAChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct VgaWriter {
    col: usize,
    row: usize,
    buffer: Unique<Buffer>,
    color: ColorCode
}

impl VgaWriter 
{
    pub fn new(color: ColorCode) -> VgaWriter
    {
        VgaWriter 
        {
            col: 0,
            row: 0,
            buffer: unsafe { Unique::new(0xb8000 as *mut _) },
            color: color
        }
    }

    pub fn write_char(&mut self, letter: u8)
    {
        match letter 
        {
            b'\n'   => self.new_line(),
            letter  => 
            {
                let col = self.col;
                let row = self.row;
                self.buffer().chars[row][col] = VGAChar {
                    text: letter,
                    color: self.color
                };
                self.col += 1;
            }

        }
    }

    pub fn write_str(&mut self, string: &str)
    {
        for letter in string.bytes()
        {
            self.write_char(letter);
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.get_mut() }
    }

    fn new_line(&mut self) { self.row += 1 }
}

// Make our function formattable
impl ::core::fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        self.write_str(s);
        Ok(())
    }
}


// Unfortunately, we can't use the VgaWriter::new function here.
pub static BUFFER: Mutex<VgaWriter> = Mutex::new(VgaWriter {
        col: 0, 
        row: 0,
        buffer: unsafe { Unique::new(0xb8000 as *mut _) },
        color: DEF_COLOR
    });

// Prints out the given arguments to the screen.
macro_rules! print {
    ($($arg:tt)*) => 
        ({
            use core::fmt::Write;
            let mut b = $crate::arch::vga::BUFFER.lock();
            b.write_fmt(format_args!($($arg)*)).unwrap();        
        });
}
