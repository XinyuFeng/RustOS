#![no_std] // not relying std library
#![no_main]

use core::panic::PanicInfo;

// this funciton is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
