{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };
      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        targets = [ 
          "thumbv7em-none-eabihf"
          "thumbv6m-none-eabi"
        ];
        extensions = [ 
            "rust-src" 
            "rust-analyzer" 
        ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          rustToolchain
          pkgs.probe-rs-tools
          pkgs.elf2uf2-rs
          pkgs.flip-link
          pkgs.pkg-config
          pkgs.openssl
          pkgs.libudev                # For probe-rs USB detection
          pkgs.libusb                 # For probe-rs
          pkgs.defmt-tools            # For defmt logging
        ];
        shellHook = ''
          export RUST_LOG=debug
          # For Embassy and CYW43 development
          export CARGO_TARGET_THUMBV7EM_NONE_EABIHF_RUNNER="probe-rs run --chip RP2350"
        '';
      };
    };
}