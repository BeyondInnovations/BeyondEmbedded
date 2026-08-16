# Raspberry Pi Pico 2 Rust template

A `cargo-generate` template for an Embassy-based Raspberry Pi Pico 2 (RP2350)
application. It targets the Arm Cortex-M33 core and includes RTT/`defmt`
logging for use with a debug probe.

## Create a project

From a checkout of this repository:

```sh
cargo generate --path pico2-rust-template --name my-pico-app
```

Or, after publishing this directory as its own Git repository:

```sh
cargo generate --git https://github.com/<owner>/<repository>.git --name my-pico-app
```

## Included

- Embassy async executor and RP2350 HAL
- Cortex-M33 linker and Cargo configuration
- `defmt` + RTT logging and `panic-probe`
- `probe-rs` VS Code launch configuration
- A Nix development shell

## Build and flash

Install the `thumbv8m.main-none-eabihf` Rust target, connect an RP2350 debug
probe, then run:

```sh
cargo run --release
```

The default Cargo runner uses `picotool`. If you use a debug probe, change the
runner in `.cargo/config.toml` to your preferred `probe-rs run` command.
