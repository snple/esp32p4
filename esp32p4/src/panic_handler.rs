//! Default panic handler: disable interrupts and park the core.

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    esp32p4_rt::interrupt::disable();
    loop {
        esp32p4_rt::asm::wfi();
    }
}
