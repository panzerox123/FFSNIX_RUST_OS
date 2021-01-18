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


pub fn writeChar(text_block: textBlock, offset: u16){
    if offset < 2000 {
        let VGA_BUFFER = 0xb8000 as *mut u8;
        unsafe{
            *VGA_BUFFER.offset(2*offset as isize) = text_block.text;
            *VGA_BUFFER.offset(2*offset as isize + 1) = ((text_block.bgColor as u8) << 4 | text_block.fgColor as u8) as u8;
        }
    }
}

pub fn writeString(text: &[u8], offset: u16){
    let mut i = 0;
    for ch in text.iter(){
        let a = textBlock{
            text: *ch,
            fgColor: Color::Black,
            bgColor: Color::White,
        };
        writeChar(a,i);
        i+=1;
    }
}