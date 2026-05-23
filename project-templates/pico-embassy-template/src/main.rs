//! SPDX-License-Identifier: MIT OR Apache-2.0
//!
//! Copyright (c) 2021–2024 The rp-rs Developers
//! Copyright (c) 2025 Raspberry Pi Ltd.
//!
//! # GPIO 'Blinky' Example with Embassy
//!
//! This application demonstrates how to control a GPIO pin on the rp2040 using Embassy.

#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

// debug_halt_once: Allow attaching a debugger before the main loop starts
// Without this, breakpoints are not hit
// Used in debug builds only, only halts once
#[inline(never)]
fn debug_halt_once() {
    unsafe {
        core::arch::asm!("bkpt #0", options(nomem, nostack, preserves_flags));
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Just for debug builds, allow attaching a debugger
    #[cfg(debug_assertions)]
    debug_halt_once();

    let mut led = Output::new(p.PIN_15, Level::Low);
    let timeout = 2000;

    info!("Hello, Pico Embassy!");
    info!("Starting main loop with {} ms timeout", timeout);
    loop {
        led.set_high();
        info!("LED on");
        Timer::after_millis(timeout).await;

        led.set_low();
        info!("LED off");
        Timer::after_millis(timeout).await;
    }
}

/// Program metadata for `picotool info`
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 5] = [
    embassy_rp::binary_info::rp_cargo_bin_name!(),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_description!(c"Blinky Example"),
    embassy_rp::binary_info::rp_cargo_homepage_url!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];
