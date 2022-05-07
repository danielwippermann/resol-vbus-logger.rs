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


### Using the SQLite logger in tabular mode

The SQLite logger needs some manual setup to work in tabular mode.

- Make sure the SQLite is disabled by setting `sqlite_tick_interval` to 0
- Start the resol-vbus-logger in debug mode:
    ```
    $ RUST_LOG=debug target/debug/logger
    ```
- The tool will start the "packet settling" phase. In that phase it tries to identify, which packets are transmitted over the VBus. At the end of the process, a list of packets and packet field IDs is printed:
    ```
    ...
    [2022-05-07T05:00:39Z DEBUG logger] Received new packet, need to resettle...
    [2022-05-07T05:00:39Z DEBUG logger] Received new packet, need to resettle...
    [2022-05-07T05:00:40Z DEBUG logger] Settling: 1 / 6 -> 16.67%
    [2022-05-07T05:00:40Z DEBUG logger] Settling: 2 / 6 -> 33.33%
    [2022-05-07T05:00:41Z DEBUG logger] Settling: 3 / 6 -> 50.00%
    [2022-05-07T05:00:41Z DEBUG logger] Settling: 4 / 6 -> 66.67%
    [2022-05-07T05:00:42Z DEBUG logger] Settling: 5 / 6 -> 83.33%
    [2022-05-07T05:00:42Z DEBUG logger] Settling: 6 / 6 -> 100.00%
    [2022-05-07T05:00:43Z DEBUG logger] Settled ["00_0010_7E11_10_0100", "00_6651_7E11_10_0200"]
    [2022-05-07T05:00:43Z DEBUG logger]   - 00_0010_7E11_10_0100_000_2_0: DeltaSol MX [Regler]: Temperatur Sensor 1
    [2022-05-07T05:00:43Z DEBUG logger]   - 00_0010_7E11_10_0100_002_2_0: DeltaSol MX [Regler]: Temperatur Sensor 2
    [2022-05-07T05:00:43Z DEBUG logger]   - 00_0010_7E11_10_0100_004_2_0: DeltaSol MX [Regler]: Temperatur Sensor 3
    ...
    ```
- You can stop the logger after getting the output above
- Open the `config.toml` file and transfer all packet field IDs you are interested in into the `sqlite_fields` array
- Make sure that the `sqlite_statement` matches your `sqlite_fields` entries:
    - The amount of columns in the first set of parentheses must match the amount of `sqlite_field` entries
    - The amount of question marks in the second set of parentheses must match the amout of `sqlite_field` entries
- Enable SQLite logging by setting `sqlite_tick_interval` to a value greater than 0
- Save the `config.toml`
- Open the SQLite file references in the `sqlite_filename` configuration value
- Create a table that matches your `sqlite_statement`, e.g.
    ```
    CREATE TABLE data(id ROWID, time TEXT, temp1 REAL, temp2 REAL, temp3 REAL, temp4 REAL, temp5 REAL, temp6 REAL, pump1 REAL, pump2 REAL, pump3 REAL, heat REAL);
    ```
- Start the logger again (does not have to be in debug mode, but might be helpful)
- Wait for the SQLite database to fill


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
