#![no_std] // To disable linking of Rust Standard library
#![no_main] // To disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        
    }
}


static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // To not change the name of the function into unique hash
pub extern "C" fn _start() -> ! {
    // Function if entry point,since the linker looks for a function 
    // named `_start` by default
    
    // vga_buffer is located at address 0xb8000
    let vga_buffer = 0xb8000 as *mut u8; // integer into raw pointer

    for (i,&byte) in HELLO.iter().enumerate() {
        unsafe {
            // Offset method to write the string byte and color byte
            *vga_buffer.offset(i as isize * 2 ) = byte; 
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // light cyan
        }
    }



    loop {}
    
}
