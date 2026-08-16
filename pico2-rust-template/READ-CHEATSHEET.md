# Raspberry Pi Pico 2 cheat sheet

## Enter dev shell

```bash
nix develop
```

## Generate a project

```bash
cargo generate --path pico2-rust-template --name my-pico-app
```

```bash
cargo check
cargo run --release
```

## diagnosing

```bash
# unplug the debug probe's USB cable entirely, wait a few seconds, replug it
probe-rs list        # confirm it's detected fresh
cargo embed           # or your run command

# in case further information is needed
probe-rs info --verbose
```
