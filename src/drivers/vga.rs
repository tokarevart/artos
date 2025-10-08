#![allow(dead_code)]

use core::fmt::Write;

use crate::sync::Mutex;

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Color {
    #[default]
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

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug)]
#[repr(transparent)]
pub struct ColorCode(pub u8);

impl ColorCode {
    #[inline]
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | foreground as u8)
    }
}

#[derive(PartialEq, Eq, Default, Clone, Copy, Debug)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

pub const BUFFER_ADDR: usize = 0xb8000;
pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    position: 0,
    color_code: ColorCode::new(Color::White, Color::Black),
    buffer: BUFFER_ADDR as _,
});

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Buffer {
    pub chars: [ScreenChar; BUFFER_WIDTH * BUFFER_HEIGHT],
}

impl Default for Buffer {
    #[inline]
    fn default() -> Self {
        Self {
            chars: [Default::default(); BUFFER_WIDTH * BUFFER_HEIGHT],
        }
    }
}

pub struct Writer {
    pub position: usize,
    pub color_code: ColorCode,
    pub buffer: *mut Buffer,
}

unsafe impl Send for Writer {}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        assert!(self.position < BUFFER_WIDTH * BUFFER_HEIGHT);

        match byte {
            b'\n' => self.new_line(),
            byte => {
                let color_code = self.color_code;
                unsafe {
                    core::ptr::write_volatile(
                        &raw mut (*self.buffer).chars[self.position],
                        ScreenChar {
                            ascii_character: byte,
                            color_code,
                        },
                    )
                };
                self.position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        self.position = (self.position + 1).next_multiple_of(BUFFER_WIDTH);
    }
}

impl Write for Writer {
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

#[macro_export]
macro_rules! vga_println {
    () => {
        $crate::vga_print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::vga_print!("{}\n", ::core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! vga_try_println {
    () => {
        $crate::vga_try_print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::vga_try_print!("{}\n", ::core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => {
        $crate::vga::print(::core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! vga_try_print {
    ($($arg:tt)*) => {
        $crate::vga::try_print(::core::format_args!($($arg)*))
    };
}

pub fn print(args: core::fmt::Arguments) {
    assert!(try_print(args));
}

#[must_use]
pub fn try_print(args: core::fmt::Arguments) -> bool {
    if let Some(mut x) = WRITER.try_lock() {
        x.write_fmt(args).unwrap();
        true
    } else {
        false
    }
}
