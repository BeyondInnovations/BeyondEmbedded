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
            probe-rs-tools
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

          shellHook = ''
            echo "Starting Rust embedded dev environment for Raspberry Pi Pico 2 W (RP2350)!"
            echo "Cargo Version: $(cargo --version)"
            echo "Probe RS Version: $(probe-rs --version)"
            echo "PicoTool Version: $(picotool version)"
          '';

        };
      });
}