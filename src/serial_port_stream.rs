use std::io::{Error, ErrorKind, Read, Result, Write};
use std::time::Duration;

use serialport::{SerialPort};

use resol_vbus::ReadWithTimeout;


pub struct SerialPortStream(Box<dyn SerialPort>);


impl SerialPortStream {
    pub fn new(port: Box<dyn SerialPort>) -> SerialPortStream {
        SerialPortStream(port)
    }
}


impl Read for SerialPortStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf)
    }
}


impl ReadWithTimeout for SerialPortStream {
    fn read_with_timeout(&mut self, buf: &mut [u8], timeout: Option<Duration>) -> Result<usize> {
        if timeout.is_none() {
            return Err(Error::new(ErrorKind::Other, "Must supply a timeout"));
        }
        if let Err(err) = self.0.set_timeout(timeout.unwrap()) {
            return Err(Error::new(ErrorKind::Other, err.description));
        }
        self.0.read(buf)
    }
}


impl Write for SerialPortStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}
