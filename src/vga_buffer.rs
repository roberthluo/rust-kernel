use core::ptr::Unique;
use volatile::Volatile;
use core::fmt;
use spin::Mutex;



pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
    buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
});


#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer().chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.as_mut() }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][col].read();
                buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[row][col].write(blank);
        }
    }

    pub fn print_something() {
        let mut writer = Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::LightGreen, Color::Black),
            //Creates new Writer that poin ts to VGA buffer at 0xb8000
            buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
        };

        writer.write_byte(b'H');
    }

    // print whole string by converting to bytes and writing one at each time
    pub fn write_str(&mut self, s:&str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }
    
}


impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

/*
 * Split because it causes deadlocking.
 * We need to evaluate the arugments before locking the writer
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::vga_buffer::WRITER.lock();
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}
*/
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}


