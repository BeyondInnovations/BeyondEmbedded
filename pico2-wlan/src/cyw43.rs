// defmt Logging
use defmt_rtt as _;


use embassy_executor::Spawner;

use embassy_rp::Peri;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::Pio;

use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};

use static_cell::StaticCell;

// defmt Logging
use defmt::{info, unwrap};

use embassy_net::{Config, StackResources};

use embassy_rp::dma;

use crate::Irqs;

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<
        'static,
        cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>,
        // cyw43::Cyw43439,
    >,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}


pub async fn init_cyw43(
    spawner: Spawner,
    pio0: Peri<'static, PIO0>,
    dma_ch0: Peri<'static, DMA_CH0>,
    pin23: Peri<'static, embassy_rp::peripherals::PIN_23>,
    pin24: Peri<'static, embassy_rp::peripherals::PIN_24>,
    pin25: Peri<'static, embassy_rp::peripherals::PIN_25>,
    pin29: Peri<'static, embassy_rp::peripherals::PIN_29>,
) -> (cyw43::NetDriver<'static>, cyw43::Control<'static>) 
{
    info!("Start Initializing CYW43");

    let fw = cyw43::aligned_bytes!("../../cyw43-firmware/43439A0.bin");
    let clm = cyw43::aligned_bytes!("../../cyw43-firmware/43439A0_clm.bin");
    let nvram = cyw43::aligned_bytes!("../../cyw43-firmware/nvram_rp2040.bin");

    let pwr = Output::new(pin23, Level::Low);
    let cs = Output::new(pin25, Level::High);
    let mut pio = Pio::new(pio0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        pin24,
        pin29,
        dma::Channel::new(dma_ch0, Irqs),
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw, nvram).await;

    spawner.spawn(unwrap!(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    info!("Finished Initializing CYW43");
    
    (net_device, control)
}