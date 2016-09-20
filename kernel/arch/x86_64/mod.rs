// This is just an example. Please, Please, Please, Please do not ever use this. 
pub fn print_char(letter: u8, pos: usize) {
    let buffer = (0xb8000 + (pos * 2)) as *mut _;
    unsafe {
        *buffer = letter;
    }
}
