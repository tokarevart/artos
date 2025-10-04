use core::fmt::Write;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    White = 15,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct ColorCode(pub u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | foreground as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

pub const BUFFER_ADDR: usize = 0xb8000;
pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

pub static mut BUFFER: *mut Buffer = BUFFER_ADDR as *mut Buffer;

// TODO: make it sound with some guard object
pub fn buffer() -> &'static mut Buffer {
    unsafe { &mut *BUFFER }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Buffer {
    pub chars: [ScreenChar; BUFFER_WIDTH * BUFFER_HEIGHT],
}

pub struct Writer<'a> {
    pub position: usize,
    pub color_code: ColorCode,
    pub buffer: &'a mut Buffer,
}

impl Writer<'_> {
    pub fn write_byte(&mut self, byte: u8) {
        assert!(self.position < BUFFER_WIDTH * BUFFER_HEIGHT);

        match byte {
            b'\n' => self.new_line(),
            byte => {
                let color_code = self.color_code;
                self.buffer.chars[self.position] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        self.position = (self.position + 1).next_multiple_of(BUFFER_WIDTH);
    }
}

impl Write for Writer<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }

        Ok(())
    }
}
