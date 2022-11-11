#![no_std] // To disable linking of Rust Standard library
#![no_main] // To disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(gh0St_OS::test_runner)]
#![reexport_test_harness_main = "test_main"] // name of the test framework entry function . call it from our _start entry point. Because The custom test frameworks feature generates a main function that calls test_runner, but this function is ignored because we use the #[no_main] attribute and provide our own entry point.

use core::panic::PanicInfo;
use gh0St_OS::println;

#[no_mangle] // To not change the name of the function into unique hash
pub extern "C" fn _start() -> ! {
    // Function if entry point,since the linker looks for a function 
    // named `_start` by default
    
    println!("Hello World{}","!");


    #[cfg(test)] // Calling `main` function created for test
    test_main();

    loop {} // Ensures that it never returns
    
}

// This function is called on panic 
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}",_info);

    loop {}
}

// panic handler in test mode 
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
   gh0St_OS::test_panic_handler(_info);
}

