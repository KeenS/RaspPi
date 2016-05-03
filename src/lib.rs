#![feature(lang_items, asm, core_intrinsics)]
#![no_std]
use core::intrinsics::volatile_store;

mod gpio;
#[no_mangle]
pub extern fn main() {
    use gpio::*;

    let gpio = GPIO_BASE as *const u32;
    let init   = unsafe {gpio.offset(LED_GPFSEL) as *mut u32};
    let led_on = unsafe { gpio.offset(LED_GPSET) as *mut u32 };
    let led_off = unsafe { gpio.offset(LED_GPCLR) as *mut u32 };

    unsafe{
        volatile_store(init, *(init) | 1 << LED_GPFBIT);
    }

    unsafe {
        volatile_store(led_on, 1 << LED_GPIO_BIT);
    }
    loop {
        unsafe {
            volatile_store(led_off, 1 << LED_GPIO_BIT);
        }
        for _ in 1..500000 {
            unsafe { asm!(""); }
        }

        unsafe {
            volatile_store(led_on, 1 << LED_GPIO_BIT);
        }
        for _ in 1..500000 {
            unsafe { asm!(""); }
        }
    }

}


#[no_mangle]
pub extern fn _sbrk() {}
#[no_mangle]
pub extern fn _exit() {}
#[no_mangle]
pub extern fn _kill() {}
#[no_mangle]
pub extern fn _getpid() {}


#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() {}
