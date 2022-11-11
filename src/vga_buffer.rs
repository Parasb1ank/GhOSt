#[allow(dead_code)] // the compiler would issue a warning for each unused variant
#[derive(Debug,Clone,Copy,PartialEq,Eq)] // Enable semantics
#[repr(u8)] // Each enum variant is stored as `u8` (8 bits)
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

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(transparent)] // For same data layout as `u8`
struct ColorCode(u8); // contains the full color byte

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        // first four bits define the foreground color,
        // the next three bits the background color
        ColorCode( (background as u8) << 4 | (foreground as u8)  )
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(C)] //the struct’s fields are laid out exactly like in a C struct and thus guarantees the correct field ordering
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

// The height of the text buffer (normally 25 lines).
const BUFFER_HEIGHT: usize = 25;
// The width of the text buffer (normally 80 columns).
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;
// A structure representing the VGA text buffer
struct Buffer {
    chars: [ [Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT ],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}


impl Writer {
    pub fn write_byte(&mut self,byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write( ScreenChar {
                    ascii_character: byte,
                    color_code,
                } );
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    //  clears a row by overwriting all of its characters with a space character.
    fn clear_row(&mut self, row:usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank); // writes to last line
        }
    }

    pub fn write_string(&mut self, s:&str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline 
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Not part of printable ASCII range 
                _ => self.write_byte(0xfe), // we print : `■`  
            }
        }
    }
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


use lazy_static::lazy_static; // At runtime intializes value without computing

// To get synchronized interior mutability, we use basic kind of Mutex: SpinLock. 
// Instead of blocking, the threads simply try to lock it again and again in a tight loop
//thus burning CPU time until the mutex is free again. It requires no operating system features
use spin::Mutex; 

// A global `Writer` instance that can be used for printing to the VGA text buffer
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { 
        column_position: 0, 
        color_code: ColorCode::new(Color::Pink, Color::Black), 
        buffer: unsafe { &mut *( 0xb8000 as *mut Buffer ) },
    } );
}

// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*))); // use our _print method
}


// Like the `println!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n",format_args!($($arg)*)));
}

#[doc(hidden)] // this a private implementation detail, so hide from generated doc
// Prints the given formatted string to the VGA text buffer through the global `WRITER` instance.
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}




/* Tests */ 
#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}",s);
    for (i,c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT-2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
