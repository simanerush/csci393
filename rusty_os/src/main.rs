#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// this function is the main entrypoint
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
