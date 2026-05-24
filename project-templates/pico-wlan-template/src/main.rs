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

// use cyw43::aligned_bytes;
// use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
// //use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};

// use defmt::*;
// use embassy_executor::Spawner;

// use embassy_rp::gpio;
// use gpio::{Level, Output};

// use {defmt_rtt as _, panic_probe as _};

// use core::str::from_utf8;

// use embassy_rp::clocks::RoscRng;
// use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIO0};
// use embassy_rp::pio::{InterruptHandler, Pio};

// use embassy_rp::{bind_interrupts, dma};
// use embassy_time::{Duration, Timer};

// use reqwless::client::HttpClient;
// // Uncomment these for TLS requests:
// // use reqwless::client::{HttpClient, TlsConfig, TlsVerify};
// use reqwless::request::Method;
// use serde::Deserialize;
// use serde_json_core::from_slice;
// use static_cell::StaticCell;
// use {defmt_rtt as _, panic_probe as _};

use cyw43::aligned_bytes;
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, dma};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
});

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Just for debug builds, allow attaching a debugger
    #[cfg(debug_assertions)]
    debug_halt_once();

    info!("Hello, Pico WLAN!");

    // Network
    let fw = aligned_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = aligned_bytes!("../cyw43-firmware/43439A0_clm.bin");
    let nvram = aligned_bytes!("../cyw43-firmware/nvram_rp2040.bin");

    // To make flashing faster for development, you may want to flash the firmwares independently
    // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
    //     probe-rs download 43439A0.bin --binary-format bin --chip RP2040 --base-address 0x10100000
    //     probe-rs download 43439A0_clm.bin --binary-format bin --chip RP2040 --base-address 0x10140000
    //let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) };
    //let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        dma::Channel::new(p.DMA_CH0, Irqs),
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (_net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw, nvram).await;
    spawner.spawn(unwrap!(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // end Network

    let mut led = Output::new(p.PIN_15, Level::Low);
    let timeout = Duration::from_secs(2);

    info!("Starting main loop with {} ms timeout", timeout);
    loop {
        led.set_high();
        info!("LED on");
        control.gpio_set(0, false).await;

        Timer::after(timeout).await;

        led.set_low();
        info!("LED off");
        control.gpio_set(0, true).await;
        Timer::after(timeout).await;
    }
}

// debug_halt_once: Allow attaching a debugger before the main loop starts
// Without this, breakpoints are not hit
// Used in debug builds only, only halts once
#[inline(never)]
fn debug_halt_once() {
    unsafe {
        core::arch::asm!("bkpt #0", options(nomem, nostack, preserves_flags));
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
