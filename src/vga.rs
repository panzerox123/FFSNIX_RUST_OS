#[derive(Debug)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magneta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12 ,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub struct textBlock {
    pub text: u8,
    pub fgColor: Color,
    pub bgColor: Color,
}


pub fn writeByte(text_block: textBlock){
    let VGA_BUFFER = 0xb8000 as *mut u8;
    unsafe{
        *VGA_BUFFER.offset(0) = text_block.text;
        *VGA_BUFFER.offset(1) = ((text_block.bgColor as u8) << 4 | text_block.fgColor as u8) as u8;
    }
}
