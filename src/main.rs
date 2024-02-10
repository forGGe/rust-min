#![no_main]
#![no_std]

use core::arch::asm;
use core::panic::PanicInfo;

//------------------------------------------------------------------------------

// Overriding/removing functions below is
// a potential future optimization for debug builds

// #[lang = "panic_fmt"]
// extern "C" fn rust_begin_panic() -> ! {
//     unsafe { intrinsics::abort() }
// }

// #[lang = "eh_unwind_resume"]
// extern "C" fn rust_eh_unwind_resume() {}

// #[no_mangle]
// pub extern "C" fn rust_eh_register_frames() {}

// #[no_mangle]
// pub extern "C" fn rust_eh_unregister_frames() {}


// Overriding the panic hanlder alone given enourmoous optimization
// in release builds

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

//------------------------------------------------------------------------------

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

//------------------------------------------------------------------------------

fn set_reg(base: u32, offt: u32, bitlen: u8, bitidx: u8, bitstate: u8) {
    let mask_low = (1 << bitidx) - 1;
    let mask_high = (1 << (bitidx + bitlen)) - 1;

    let mask: u32 = mask_high & (!mask_low);
    let value: u32 = ((bitstate as u32) << bitidx) & mask;

    let addr = base + offt;
    let reg = addr as *mut u32;

    unsafe {
        let mut regval: u32 = *reg;
        regval &= !mask;
        regval |= value;
        *reg = regval;
    }
}

// Completely abstract delay, cycles doesn't meen much in the real world
fn delay(cycles: u32) {
    // loops are optimized differently so to give some consistency the counter
    // must be adjusted

    let proper_cycles = if cfg!(debug_assertions) {
        cycles >> 1
    } else {
        cycles * 4
    };

    for _ in 0..proper_cycles {
        unsafe {
            asm!("nop");
        }
    }
}

//------------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // Enable RCC clocks for port D

    let rcc_base = 0x40021000;
    let rcc_ctrl_offt = 0x14;
    let rcc_gpiod_en_offt = 20;
    let rcc_gpiod_en_sz = 1;

    set_reg(
        rcc_base,
        rcc_ctrl_offt,
        rcc_gpiod_en_sz,
        rcc_gpiod_en_offt,
        0b1,
    );

    // Set type of pin D2 to output

    let gpiod_base = 0x48000C00;
    let gpiod_ctrl_offt = 0x0;
    let gpiod_type_offt = 4;
    let gpiod_type_sz = 2;

    set_reg(
        gpiod_base,
        gpiod_ctrl_offt,
        gpiod_type_sz,
        gpiod_type_offt,
        0b01,
    );

    // can't return so we go into an infinite loop here
    loop {
        let gpiod_odr_offt = 0x14;
        let gpiod_odr_sz = 1;
        let gpiod_odr1_offt = 2;

        set_reg(
            gpiod_base,
            gpiod_odr_offt,
            gpiod_odr_sz,
            gpiod_odr1_offt,
            0b1,
        );

        delay(80000);

        set_reg(
            gpiod_base,
            gpiod_odr_offt,
            gpiod_odr_sz,
            gpiod_odr1_offt,
            0b0,
        );

        delay(80000);
    }
}

