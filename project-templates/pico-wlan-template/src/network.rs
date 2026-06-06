use cyw43::{JoinOptions, aligned_bytes};
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, dma};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use cyw43::NetDriver;
use embassy_net::{Config, StackResources};
use embassy_rp::clocks::RoscRng;

use crate::config::WIFI_CONFIG;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
});

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>>,
) -> ! {se embassy_executor::Spawner;
    runner.run().await
}

use embassy_rp::Peri;
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_24, PIN_25, PIN_29, PIO0};

/// Think of PIN_23 as the identity/type-level name of the peripheral,
/// and Peri<'static, PIN_23> as the actual ownership token you receive from embassy_rp::init(...). Most Embassy APIs consume the ownership token, not the marker type by itself.

pub async fn init_network(
    spawner: Spawner,
    pin_23: Peri<'static, PIN_23>,
    pin_24: Peri<'static, PIN_24>,
    pin_25: Peri<'static, PIN_25>,
    pin_29: Peri<'static, PIN_29>,
    pio0: Peri<'static, PIO0>,
    dma_ch0: Peri<'static, DMA_CH0>,
) -> (NetDriver<'static>, cyw43::Control<'static>) {
    let fw = aligned_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = aligned_bytes!("../cyw43-firmware/43439A0_clm.bin");
    let nvram = aligned_bytes!("../cyw43-firmware/nvram_rp2040.bin");

    // To make flashing faster for development, you may want to flash the firmwares independently
    // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
    //     probe-rs download 43439A0.bin --binary-format bin --chip RP2040 --base-address 0x10100000
    //     probe-rs download 43439A0_clm.bin --binary-format bin --chip RP2040 --base-address 0x10140000
    //let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) };
    //let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

    info!("Initializing network...");

    let pwr = Output::new(pin_23, Level::Low);
    let cs = Output::new(pin_25, Level::High);
    let mut pio = Pio::new(pio0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        pin_24,
        pin_29,
        dma::Channel::new(dma_ch0, Irqs),
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    // let (_net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw, nvram).await;
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw, nvram).await;

    spawner.spawn(unwrap!(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    (net_device, control)
}

pub async fn scan_networks(control: &mut cyw43::Control<'static>) {
    info!("Scanning for networks!");

    let mut scanner = control.scan(Default::default()).await;
    while let Some(bss) = scanner.next().await {
        if let Ok(ssid_str) = str::from_utf8(&bss.ssid) {
            info!("scanned {} == {:x}", ssid_str, bss.bssid);
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

pub async fn connect_network(
    spawner: Spawner,
    control: &mut cyw43::Control<'static>,
    net_device: cyw43::NetDriver<'static>,
) {
    info!("Joining network...");

    let config = Config::dhcpv4(Default::default());
    //let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
    //    address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 69, 2), 24),
    //    dns_servers: Vec::new(),
    //    gateway: Some(Ipv4Address::new(192, 168, 69, 1)),
    //});

    let mut rng = RoscRng;

    // Generate random seed
    let seed = rng.next_u64();

    // Init network stack
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed,
    );

    spawner.spawn(unwrap!(net_task(runner)));

    let wifi_ssid = WIFI_CONFIG.ssid;
    let wifi_password = WIFI_CONFIG.password;

    // let mut scanner = control.scan(Default::default()).await;
    // while let Some(bss) = scanner.next().await {
    //     if let Ok(ssid_str) = str::from_utf8(&bss.ssid) {
    //         info!("scanned {} == {:x}", ssid_str, bss.bssid);
    //     }
    // }

    info!("connecting to wifi network '{}'", wifi_ssid);

    loop {
        let options = JoinOptions::new(wifi_password.as_bytes());
        match control.join(wifi_ssid, options).await {
            Ok(_) => {
                info!("connected to wifi network");
                break;
            }
            Err(err) => match err.status {
                1 => info!("connection attempt failed (generic failure), retrying..."),
                2 => info!("no matching SSID found (out of range / SSID not in scan), retrying..."),
                3 => info!("authentication failed (bad password / credentials), retrying..."),
                _ => info!("unknown status code, retrying..."),
            },
        }
    }

    info!("waiting for link...");
    stack.wait_link_up().await;

    info!("waiting for DHCP...");
    stack.wait_config_up().await;

    // And now we can use it!
    info!("Stack is up!");
}
