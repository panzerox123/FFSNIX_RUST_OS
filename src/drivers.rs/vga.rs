#[derive(Debug)]
#[allow(dead_code)]
pub enum Color{
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

pub static VGA_BUFFER: *mut u8 = 0xb8000;

struct textBlock {
    text: u8,
    color: Color,
}


fn writeByte(text_block: textBlock){
    
    unsafe{
        *VGA_BUFFER.offset(0) = textBlock.text;
        *VGA_BUFFER.offset(1) = textBlock.color;
    }
}




struct Text{
    character: u8,
    color_
}

impl 