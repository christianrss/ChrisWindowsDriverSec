#![no_std]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "system" fn driver_entry() -> u32 {
    0 /* NT_STATUS SUCCESS */
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}