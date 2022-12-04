use core::fmt;

#[allow(unused)]
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Color {
  Black = 0x0,
  White = 0xF,
  Blue = 0x1,
  BrightBlue = 0x9,
  Green = 0x2,
  BrightGreen = 0xA,
  Cyan = 0x3,
  BrightCyan = 0xB,
  Red = 0x4,
  BrightRed = 0xC,
  Magenta = 0x5,
  BrightMagenta = 0xD,
  Brown = 0x6,
  Yellow = 0xE,
  Gray = 0x7,
  DarkGray = 0x8
}

pub struct Cursor {
  position: isize,
  foreground: Color,
  background: Color,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            position: 0,
            foreground: Color::White,
            background: Color::Red,
        }
    }

  pub fn color(&self) -> u8 {
    let fg = self.foreground as u8;
    let bg = (self.background as u8) << 4;
    fg | bg
  }

  pub fn print(&mut self, text: &[u8]) {
    let color = self.color();

    let framebuffer = 0xb8000 as *mut u8;

    for &character in text {
      unsafe {
        framebuffer.offset(self.position)
            .write_volatile(character);
        framebuffer.offset(self.position + 1)
            .write_volatile(color);
      }
      self.position += 2;
    }
  }

  pub fn to_begin(&mut self) {
    self.position = 0;
  }
}

impl fmt::Write for Cursor {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.print(s.as_bytes());
    Ok(())
  }
}