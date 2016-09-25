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

pub const DEF_COLOR: ColorCode = ColorCode::new(Color::LightGreen, Color::Black);

#[repr(C)] #[derive(Clone, Copy)]
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

    pub fn write_char(&mut self, letter: u8)
    {
        match letter 
        {
            b'\n'   => self.new_line(),
            letter  => 
            {
                let col = self.col;
                let row = self.row;
                if col >= BUFFER_WIDTH {
                    self.new_line();
                }
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

    fn new_line(&mut self) {
        self.row += 1;
        if self.row >= BUFFER_HEIGHT
        {
            self.scroll()
        }
        self.col = 0;
    }

    pub fn scroll(&mut self)
    {
        for row in 0 .. (BUFFER_HEIGHT - 1) // Iterate through all the rows
        { 
            self.buffer().chars[row] = self.buffer().chars[row + 1];
        }
        let space = VGAChar{ text: ' ' as u8, color: self.color }; // A default space
        self.buffer().chars[BUFFER_HEIGHT - 1] = [space; 80]; // Fill the last row with spaces
        self.row = BUFFER_HEIGHT - 1; // Put cursor on last row

    }

    pub fn position(&mut self, x: usize, y: usize)
    {
        if (x >= BUFFER_WIDTH) || (y >= BUFFER_HEIGHT)  {
            self.col = 0;
            self.row = 0;
        }
        self.col = x;
        self.row = y;
    }

    pub fn clear(&mut self)
    {
        let space = VGAChar { text: ' ' as u8, color: DEF_COLOR };
        let buffer = [[space; BUFFER_WIDTH]; BUFFER_HEIGHT];
        self.buffer().chars = buffer;
        self.position(0, 0);
    }

    pub fn set_color(&mut self, color: ColorCode)
    {
        self.color = color;
    }
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


pub fn init_vga()
{
    BUFFER.lock().clear();
}

