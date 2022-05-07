// This is part of resol-vbus.rs.
// Copyright (c) 2017, Daniel Wippermann.
// See README.md and LICENSE.txt for details.

//! # resol-vbus-logger.rs
//!
//! A Rust application that uses the resol-vbus.rs library to log and
//! visualize RESOL VBus data.
//!
//!
//! ## Features
//!
//! - Connected either to a serial port or VBus-over-TCP device
//! - Writes data to CSV file at configurable intervals
//! - Renders a PNG containing data at configurable intervals
//! 
//! 
//! ## First-time setup
//! 
//! ```
//! # Change to the directory where the project should be stored
//! cd <path to directory>
//! 
//! # Clone the project
//! git clone https://github.com/danielwippermann/resol-vbus-logger.rs
//! 
//! # Change into the project directory
//! cd resol-vbus-logger.rs
//! 
//! # Copy the `config.toml.example` to `config.toml`
//! cp config.toml.example config.toml
//! 
//! # Edit the `config.toml`
//! ```
//! 
//! 
//! ## Usage
//! 
//! ```
//! # Change to the project directory
//! cd <path to directory>/resol-vbus-logger.rs
//! 
//! # Run the application from the folder where the `config.toml` is located.
//! cargo run
//! ```
//!  

#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(warnings)]


extern crate env_logger;
extern crate image;
extern crate imageproc;
#[macro_use]
extern crate log;
extern crate resol_vbus;
extern crate rusttype;
#[macro_use]
extern crate serde_derive;
extern crate serialport;
extern crate sqlite;
extern crate toml;


mod config;
mod csv_generator;
mod error;
mod live_data_text_generator;
mod png_generator;
mod serial_port_stream;
mod sqlite_logger;
mod tick_source;
mod timestamp_file_writer;


use std::io::{Read, Write};
use std::net::TcpStream;

use resol_vbus::{
    chrono::prelude::*,
    Data,
    DataSet,
    Header,
    LiveDataStream,
    Packet,
    ReadWithTimeout,
    TcpConnector,
    ToPacketId,
};


use config::Config;
use csv_generator::CsvGenerator;
use error::{Error, Result};
use live_data_text_generator::LiveDataTextGenerator;
use png_generator::PngGenerator;
use serial_port_stream::SerialPortStream;
use sqlite_logger::SqliteLogger;
use tick_source::TickSource;


fn stream_live_data<R: Read + ReadWithTimeout, W: Write>(config: &Config, mut lds: LiveDataStream<R, W>) -> Result<()> {
    let mut data_set = DataSet::new();

    for packet_id in config.known_packet_ids.iter() {
        let packet_id = packet_id.to_packet_id()?;
        let packet = Packet {
            header: Header {
                timestamp: UTC::now(),
                channel: packet_id.0,
                destination_address: packet_id.1,
                source_address: packet_id.2,
                protocol_version: 0x10,
            },
            command: packet_id.3,
            frame_count: 0,
            frame_data: [0; 508],
        };
        data_set.add_data(Data::Packet(packet));
    }

    let mut data_set_is_settled = false;
    let mut data_set_settled_max_count = data_set.len() * 3;
    let mut data_set_settled_count = 0;

    debug!("Initializing PNG");
    let png_generator = PngGenerator::from_config(&config)?;
    debug!("Initializing CSV");
    let mut csv_generator = CsvGenerator::from_config(&config)?;
    debug!("Initializing Live Data Text");
    let mut live_data_text_generator = LiveDataTextGenerator::from_config(&config)?;
    debug!("Initializing SQLite");
    let mut sqlite_logger = SqliteLogger::from_config(&config)?;

    let now = UTC::now();

    debug!("Initializing tick sources");
    let mut png_tick_source = TickSource::new(config.png_tick_interval, now);
    let mut csv_tick_source = TickSource::new(config.csv_tick_interval, now);
    let mut live_data_text_tick_source = TickSource::new(config.live_data_text_tick_interval, now);
    let mut sqlite_tick_source = TickSource::new(config.sqlite_tick_interval, now);

    loop {
        let now = UTC::now();
        if png_tick_source.process(now) {
            if data_set_is_settled {
                debug!("PNG Tick");
                png_generator.generate(&data_set, &now)?;
            }
        }

        if csv_tick_source.process(now) {
            if data_set_is_settled {
                debug!("CSV tick");
                csv_generator.generate(&data_set, &now)?;
            }
        }

        if live_data_text_tick_source.process(now) {
            if data_set_is_settled {
                debug!("Live Data Text tick");
                live_data_text_generator.generate(&data_set, &now)?;
            }
        }

        if sqlite_tick_source.process(now) {
            if data_set_is_settled {
                debug!("SQlite tick");
                sqlite_logger.log(&data_set, &now)?;
            }
        }

        if let Some(data) = lds.receive(500)? {
            if !data.is_packet() {
                // nop
            } else if data_set_is_settled {
                data_set.add_data(data);
            } else {
                let len_before = data_set.len();

                data_set.add_data(data);

                let len_after = data_set.len();

                if len_before != len_after {
                    debug!("Received new packet, need to resettle...");
                    data_set_settled_max_count = len_after * 3;
                    data_set_settled_count = 0;
                } else if data_set_settled_count < data_set_settled_max_count {
                    data_set_settled_count += 1;
                    let percent = 100.0f32 * data_set_settled_count as f32 / data_set_settled_max_count as f32;
                    debug!("Settling: {} / {} -> {:.2}%", data_set_settled_count, data_set_settled_max_count, percent);
                } else {
                    data_set_is_settled = true;

                    let mut sorted_data_set = data_set.clone();
                    sorted_data_set.sort();
                    debug!("Settled {:?}", sorted_data_set.iter().map(|data| data.id_string()).collect::<Vec<_>>());
                }
            }
        }
    }
}


fn run_main() -> Result<()> {
    env_logger::init();

    debug!("Loading config");
    let config = Config::load()?;

    let channel = config.channel.unwrap_or(0);

    if let Some(ref path) = config.path {
        debug!("Using serial port");

        debug!("Connecting serial port");
        let port = serialport::new(path, 9600).open()?;

        let reader = SerialPortStream::new(port.try_clone()?);
        let writer = SerialPortStream::new(port);

        debug!("Creating live data stream");
        let lds = LiveDataStream::new(channel, 0x0020, reader, writer)?;

        stream_live_data(&config, lds)?;

        Ok(())
    } else if let Some(ref address) = config.address {
        debug!("Using TCP stream");

        debug!("Connection TCP stream");
        let stream = TcpStream::connect(address)?;

        debug!("Performing VBus-over-TCP handshake");
        let mut tcp_connector = TcpConnector::new(stream);
        tcp_connector.via_tag = config.via_tag.clone();
        tcp_connector.password = config.password.clone().unwrap_or("vbus".to_string());
        tcp_connector.channel = config.channel.clone();
        tcp_connector.connect()?;

        let reader = tcp_connector.into_inner();
        let writer = reader.try_clone()?;

        debug!("Creating live data stream");
        let lds = LiveDataStream::new(channel, 0x0020, reader, writer)?;

        stream_live_data(&config, lds)?;

        Ok(())
    } else {
        Err(Error::from("Unexpected connection method"))
    }
}


fn main() {
    run_main().unwrap();
}
