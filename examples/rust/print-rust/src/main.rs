#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

// Program metadata for `picotool info`.
// This isn't needed, but it's recommended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Pico 2 W Output Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example prints periodic RTT output on RP Pico 2 W"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_rp::init(Default::default());

    let mut count = 0;
    info!("Starting example - check the RTT output!");
    loop {
        Timer::after_millis(250).await;
        info!("ON");

        Timer::after_millis(250).await;
        info!("OFF");
        count += 1;
        // let text = format!("Count: {}", count);
        // info!("{}", text);
    }
}
