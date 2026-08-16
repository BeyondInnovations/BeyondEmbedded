# RUST Embedded essentials

## Enter dev shell

```bash
nix develop
```

## project setup

```bash
cargo generate --git https://github.com/embassy-rs/embassy --branch main examples/rp235x
```

[ImplFerris: Rust projects, learnings and experiments](https://github.com/ImplFerris)

[ImplFerris:Pico 2 Template](https://github.com/ImplFerris/pico2-template)

### Cargo generate

```bash
cargo generate --git https://github.com/embassy-rs/embassy --branch main examples/rp235x
```

[cates.io: Cargo-generate](https://crates.io/crates/cargo-generate)

```bash
# templates on github
cargo generate --git https://github.com/username-on-github/mytemplate.git

cargo generate --git https://github.com/ImplFerris/pico2-template

# or just
cargo generate username-on-github/mytemplate
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