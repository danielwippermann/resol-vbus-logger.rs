# resol-vbus-logger.rs

A Rust application that uses the resol-vbus.rs library to log and
visualize RESOL VBus data.


## Features

- Connected either to a serial port or VBus-over-TCP device
- Writes data to CSV file at configurable intervals
- Renders a PNG containing data at configurable intervals


## First-time setup

```
# Change to the directory where the project should be stored
cd <path to directory>

# Clone the project
git clone https://github.com/danielwippermann/resol-vbus-logger.rs

# Change into the project directory
cd resol-vbus-logger.rs

# Copy the `config.toml.example` to `config.toml`
cp config.toml.example config.toml

# Edit the `config.toml`
```


## Usage

```
# Change to the project directory
cd <path to directory>/resol-vbus-logger.rs

# Run the application from the folder where the `config.toml` is located.
cargo run
# or
cargo build
target/debug/logger

# To run the application with debug output, just run
cargo build
RUST_LOG=debug target/debug/logger
```


## Contributors

- [Daniel Wippermann](https://github.com/danielwippermann)


## Legal Notices

RESOL, VBus, VBus.net and others are trademarks or registered trademarks of RESOL - Elektronische Regelungen GmbH.

All other trademarks are the property of their respective owners.

The `Roboto-Regular.ttf` is distributed under the terms of the Apache License (Version 2.0): https://github.com/google/roboto


## License

`resol-vbus.rs` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See LICENSE.txt for details.
