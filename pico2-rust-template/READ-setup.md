# Rust development setup

## udev rules

```bash
curl -fsSL https://probe.rs/files/69-probe-rs.rules -o /tmp/69-probe-rs.rules
sudo cp /tmp/69-probe-rs.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
sudo usermod -aG plugdev,dialout $USER

# for Pico 2 WH
curl -fsSL https://probe.rs/files/69-probe-rs.rules -o /tmp/69-probe-rs.rules
sudo cp /tmp/69-probe-rs.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
sudo usermod -aG plugdev,dialout $USER
```

The Pico 2 W uses the RP2350 (dual-core Arm Cortex-M33 by default, though it can also run RISC-V Hazard3 cores). Here's the tailored setup.

## Updated flake.nix

```nix
{
  description = "Rust embedded dev environment - Raspberry Pi Pico 2 W (RP2350)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "llvm-tools-preview" ];
          targets = [
            "thumbv8m.main-none-eabihf"  # RP2350 Arm Cortex-M33
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            probe-rs
            cargo-binutils
            cargo-generate
            elf2uf2-rs      # convert ELF to UF2 for BOOTSEL drag-and-drop flashing
            picotool
            gdb
            pkg-config
            libusb1
            udev
            flip-link       # zero-cost stack overflow protection, common in RP2xxx projects
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.libusb1 pkgs.udev ];
        };
      });
}
```

Note: `picotool` and `elf2uf2-rs` may or may not be in your nixpkgs channel depending on freshness — if `picotool` isn't found, drop it, you likely won't need it if you're using probe-rs for flashing.

## udev rules

Same as before, but also add the Pico's BOOTSEL/mass-storage mode rule if you'll ever use UF2 drag-and-drop flashing instead of a debug probe:

```bash
curl -fsSL https://probe.rs/files/69-probe-rs.rules -o /tmp/69-probe-rs.rules
sudo cp /tmp/69-probe-rs.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
sudo usermod -aG plugdev,dialout $USER
```

## Two paths depending on how you're flashing

**A. With a debug probe (e.g. a Pico used as a Picoprobe, or a real SWD probe)** — this is the nicer dev loop (breakpoints, `defmt` logging over RTT, `cargo run` flashes directly):

`.cargo/config.toml`:
```toml
[target.thumbv8m.main-none-eabihf]
runner = "probe-rs run --chip RP235x"
rustflags = [
  "-C", "link-arg=--nmagic",
  "-C", "link-arg=-Tlink.x",
  "-C", "link-arg=-Tdefmt.x",
]

[build]
target = "thumbv8m.main-none-eabihf"
```

**B. UF2 drag-and-drop only (no probe)** — hold BOOTSEL, plug in, it mounts as USB storage, drop the `.uf2` file. Slower loop, no live debugging, but zero extra hardware.

## Getting a project started

The [embassy-rs](https://github.com/embassy-rs/embassy) examples repo has ready-made RP2350/Pico 2 W examples including WiFi via the CYW43 chip (needed for the "W" in Pico 2 W) — that's the most actively maintained option right now for async embedded Rust on this board:

```bash
cargo generate --git https://github.com/embassy-rs/embassy --branch main embassy-examples/rp
```

Alternatively, `rp235x-hal` is the lower-level, non-async HAL if you'd rather work closer to the metal.

One thing to pin down: do you have a debug probe (SWD wired up, or a second Pico as picoprobe), or are you planning to flash via BOOTSEL/UF2 only? That changes which `.cargo/config.toml` and workflow makes sense to set up first.

## Using debug probe

Good — that gives you the full debug workflow: flash, run, breakpoints, and `defmt` logging over RTT, all through `cargo run`.

## Verify probe-rs sees your setup

```bash
nix develop
probe-rs list          # should show your probe (e.g. CMSIS-DAP / Picoprobe / J-Link)
probe-rs chip list | grep -i rp235
```

You should see `RP235x` (and possibly variants like `RP2350A`/`RP2350B`). Use `RP235x` in your runner config — probe-rs will auto-detect the specific variant.

## `.cargo/config.toml`

```toml
[target.thumbv8m.main-none-eabihf]
runner = "probe-rs run --chip RP235x --protocol swd"
rustflags = [
  "-C", "link-arg=--nmagic",
  "-C", "link-arg=-Tlink.x",
  "-C", "link-arg=-Tdefmt.x",
]

[build]
target = "thumbv8m.main-none-eabihf"

[env]
DEFMT_LOG = "debug"
```

## Cargo.toml essentials

```toml
[dependencies]
embassy-rp = { version = "0.2", features = ["rp235xa", "time-driver", "critical-section-impl"] }
embassy-executor = { version = "0.6", features = ["arch-cortex-m", "executor-thread"] }
embassy-time = "0.3"
defmt = "0.3"
defmt-rtt = "0.4"
panic-probe = { version = "0.3", features = ["print-defmt"] }

[profile.release]
debug = 2
lto = true
```

`panic-probe` + `defmt-rtt` is the standard combo — panics and logs both stream back over the debug probe via RTT, printed by `probe-rs` in your terminal.

## Minimal `main.rs` to confirm the whole chain works

```rust
#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low); // onboard LED, check pin for Pico 2 W

    loop {
        info!("toggling LED");
        led.toggle();
        Timer::after_millis(500).await;
    }
}
```

⚠️ One thing to double check: on the original Pico W, the onboard LED is wired to the **CYW43 wifi chip**, not a plain GPIO — you have to drive it through the `cyw43` driver, not `PIN_25` directly. This is very likely still true on the Pico 2 W since it uses the same wifi module. I'd confirm against the current embassy `rp` examples before wiring up LED code, since getting this wrong is a common early stumbling block.

Then just:

```bash
cargo run --release
```

That should build, flash over your probe, and start streaming `defmt` logs live.

Want me to pull the exact CYW43/LED init sequence for the Pico 2 W from the current embassy examples so the LED bit above is actually correct rather than a guess?