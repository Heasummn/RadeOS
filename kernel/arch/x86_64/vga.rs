use core::ptr::Unique;
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

#[repr(C)]
struct VGAChar { 
    text: u8,
    color: ColorCode
}

pub struct Buffer {
    chars: [[VGAChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct VGA {
    col: usize,
    row: usize,
    buffer: Unique<Buffer>,
    color: ColorCode
}

impl VGA
{
    pub fn new(buffer: Unique<Buffer>, color: ColorCode) -> VGA
    {
        VGA 
        {
            col: 0,
            row: 0,
            buffer: buffer,
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

    fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.get_mut() }
    }

    fn new_line(&mut self) { self.row += 1 }
}

