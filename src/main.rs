#![no_std] // To disable linking of Rust Standard library
#![no_main] // To disable all Rust-level entry points

mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called on panic 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}",_info);

    loop {}
}



#[no_mangle] // To not change the name of the function into unique hash
pub extern "C" fn _start() -> ! {
    // Function if entry point,since the linker looks for a function 
    // named `_start` by default
    

    println!("Hello World{}","!");
    panic!("Some panic message");

    loop {} // Ensures that it never returns
    
}
