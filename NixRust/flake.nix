{
  description = "Rust embedded dev environment";

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
          # add whatever MCU targets you need, e.g.:
          targets = [
            "thumbv7em-none-eabihf"  # Cortex-M4F/M7F
            "thumbv6m-none-eabi"     # Cortex-M0/M0+
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            probe-rs        # flashing/debugging via CMSIS-DAP, ST-Link, J-Link, etc.
            cargo-binutils
            cargo-generate
            gdb
            openocd
            pkg-config
            libusb1
            udev
          ];

          # Needed for probe-rs / libusb to see USB devices
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.libusb1 pkgs.udev ];
        };
      });
}