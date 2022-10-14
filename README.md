# RS-Terrarium

RS-Terrarium is my attempt at replicating the basic features I need from
[TerrariumPI](https://github.com/theyosh/TerrariumPI).

It is written in rust, because why not and I really like playing around with it.

## Installation

## Configuration

## Development

To create a new build, make sure to have [cross](https://github.com/cross-rs/cross) installed. Then build it:

### Coverage report

To run a coverage report, install `cargo install grcov` and `rustup component add llvm-tools-preview`.

```shell
export RUSTFLAGS="-Cinstrument-coverage"
cargo build --verbose
LLVM_PROFILE_FILE="your_name-%p-%m.profraw" cargo test --verbose
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing --ignore "/*" -o lcov_html
```

Use a webserver/browser to view the files in `lcov_html`.

### Raspberry Pi Zero W

```
cross build --target arm-unknown-linux-gnueabihf
```

## TODO

* Signal handling
  We want to graceful exit on signals or Ctrl-C. Let's check https://rust-cli.github.io/book/in-depth/signals.html.
* Web-Relay
  We need to control a SONOFF relay with Tasmota or other firmwares
* Sensors
  We need multiple sensors for the terrarium to control other actors.
  - BME280
  - HC-SR04
