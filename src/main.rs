#![no_std]
#![no_main]

mod user_interface;
mod bitcoin_core;
mod bitmap_ai_layer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    user_interface::init_ui();
    bitcoin_core::init_bitcoin_core();
    bitmap_ai_layer::init_bitmap_layer();
    
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

