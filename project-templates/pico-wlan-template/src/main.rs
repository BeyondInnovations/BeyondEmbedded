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

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

mod config;
mod network;
use network::*;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Just for debug builds, allow attaching a debugger
    #[cfg(debug_assertions)]
    debug_halt_once();

    info!("Hello, Pico WLAN!");

    // Network
    let (net_device, mut control) = init_network(
        spawner, p.PIN_23, p.PIN_24, p.PIN_25, p.PIN_29, p.PIO0, p.DMA_CH0,
    )
    .await;

    scan_networks(&mut control).await;

    connect_network(spawner, &mut control, net_device).await;

    // end Network

    let mut led = Output::new(p.PIN_15, Level::Low);
    let timeout = Duration::from_secs(1);

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
