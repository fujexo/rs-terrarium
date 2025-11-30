# RS-Terrarium

RS-Terrarium is my attempt at replicating the basic features I need from
[TerrariumPI](https://github.com/theyosh/TerrariumPI).

It is written in rust, because why not and I really like playing around with it.

## Installation

```
wget -q -O /usr/share/keyrings/grafana.key https://apt.grafana.com/gpg.key
echo "deb [signed-by=/usr/share/keyrings/grafana.key] https://apt.grafana.com stable main" | sudo tee -a /etc/apt/sources.list.d/grafana.list

wget -q https://repos.influxdata.com/influxdb.key
echo '23a1c8836f0afc5ed24e0486339d7cc8f6790b83886c4c96995b88a061c5bb5d influxdb.key' | sha256sum -c && cat influxdb.key | gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/influxdb.gpg > /dev/null
echo 'deb [signed-by=/etc/apt/trusted.gpg.d/influxdb.gpg] https://repos.influxdata.com/debian stable main' | sudo tee /etc/apt/sources.list.d/influxdata.list

curl https://github.com/fujexo/rs-terrarium/packages/terrarium.deb
dpkg -i terrarium.deb
apt-get install -f

systemctl enable --now influxdb.service
systemctl enable --now grafana-server.service
systemctl enable --now terrarium.service
```

## Configuration

## Development

To create a new build, make sure to have [cross](https://github.com/cross-rs/cross) installed. Then build it:

```shell
cross build --release
```

### DEB-Package

With [cargo-deb](https://github.com/kornelski/cargo-deb), we can create a debian package for simpler installation.
This requires to have [arm-none-eabi-binutils](https://archlinux.org/packages/extra/x86_64/arm-none-eabi-binutils/) or
[aarch64-linux-gnu-binutils](https://archlinux.org/packages/extra/x86_64/aarch64-linux-gnu-binutils/) to be installed.

```shell
cargo deb --no-build --target aarch64-unknown-linux-gnu
```

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
  We want to graceful exit on signals or Ctrl-C. Let's check <https://rust-cli.github.io/book/in-depth/signals.html>.
* Web-Relay
  We need to control a SONOFF relay with Tasmota or other firmwares
* Sensors
  We need multiple sensors for the terrarium to control other actors.
  * BME280
  * HC-SR04
