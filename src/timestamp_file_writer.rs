use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Write};

use resol_vbus::chrono::{DateTime, TimeZone};


pub struct TimestampFileWriter<Tz: TimeZone> {
    filename_pattern: String,
    timestamp: DateTime<Tz>,
    timestamp_changed: bool,
    current_filename: Option<String>,
    current_file: Option<File>,
}


impl<Tz: TimeZone> TimestampFileWriter<Tz> where Tz::Offset: Display {
    pub fn new(filename_pattern: String, timestamp: DateTime<Tz>) -> TimestampFileWriter<Tz> {
        TimestampFileWriter {
            filename_pattern,
            timestamp,
            timestamp_changed: true,
            current_filename: None,
            current_file: None,
        }
    }

    pub fn set_timestamp(&mut self, timestamp: DateTime<Tz>) -> Result<bool> {
        self.timestamp = timestamp;
        self.timestamp_changed = true;

        self.check_timestamp_change()
    }

    pub fn filename(&self) -> Option<&str> {
        match self.current_filename {
            None => None,
            Some(ref filename) => Some(filename),
        }
    }

    fn check_timestamp_change(&mut self) -> Result<bool> {
        if self.timestamp_changed {
            self.timestamp_changed = false;

            let filename = Some(self.timestamp.format(&self.filename_pattern).to_string());
            if self.current_filename != filename {
                let file = OpenOptions::new().append(true).create(true).open(filename.as_ref().unwrap())?;

                self.current_filename = filename;
                self.current_file = Some(file);

                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}


impl<Tz: TimeZone> Write for TimestampFileWriter<Tz> where Tz::Offset: Display {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.check_timestamp_change()?;

        if let Some(ref mut file) = self.current_file {
            file.write(buf)
        } else {
            Err(Error::new(ErrorKind::Other, "No file created!"))
        }
    }

    fn flush(&mut self) -> Result<()> {
        if let Some(ref mut file) = self.current_file {
            file.flush()
        } else {
            Ok(())
        }
    }
}
