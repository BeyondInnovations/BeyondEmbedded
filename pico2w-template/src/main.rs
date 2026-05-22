#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use panic_probe as _;

use rp235x_hal as hal;

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

const XTAL_FREQ_HZ: u32 = 12_000_000;

#[hal::entry]
fn main() -> ! {
    info!("program start");

    let mut pac = hal::pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);
    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut led_pin = pins.gpio15.into_push_pull_output();

    let timeout = 500;
    loop {
        info!("led on");
        led_pin.set_high().unwrap();
        timer.delay_ms(timeout);
        info!("led off");
        led_pin.set_low().unwrap();
        timer.delay_ms(timeout);
    }
}
