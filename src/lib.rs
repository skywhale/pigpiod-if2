use pigpiod_if2_sys::*;
use std::os::raw::c_char;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A custom error struct for this crate.
#[derive(Debug, Clone)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

fn err_msg<S: Into<String>>(msg: S) -> Box<dyn std::error::Error> {
    Box::new(Error(msg.into()))
}

/// A client that talks to the pigpio daemon on a Pi. It's the entry point for all the GPIO
/// routines.
pub struct Pigpio {
    handle: i32,
}

impl Pigpio {
    /// Connect to the local pigpio daemon. Reserving command and notification streams.
    pub fn new() -> Result<Self> {
        let handle = unsafe { pigpio_start(0 as *mut c_char, 0 as *mut c_char) };
        if handle >= 0 {
            Ok(Self { handle })
        } else {
            Err(err_msg("Failed to connect to the local pigpiod server"))
        }
    }

    /// Returns a new `Spi` associated with the SPI device on the `channel`. Data will be
    /// transferred at `baud` bits per second. The `flags` may be used to modify the default
    /// behaviour of 4-wire operation, mode 0, active low chip select.
    /// http://abyz.me.uk/rpi/pigpio/pdif2.html#spi_open
    pub fn spi(&self, channel: u32, baud: u32, flags: u32) -> Result<Spi> {
        let spi_handle = unsafe { spi_open(self.handle, channel, baud, flags) };
        if spi_handle >= 0 {
            Ok(Spi {
                pi_handle: self.handle,
                spi_handle: spi_handle as u32,
            })
        } else {
            Err(err_msg(match spi_handle {
                PI_BAD_SPI_CHANNEL => "bad SPI channel",
                PI_BAD_SPI_SPEED => "bad SPI speed",
                PI_BAD_FLAGS => "bad flags",
                PI_NO_AUX_SPI => "no AUX SPI",
                PI_SPI_OPEN_FAILED => "SPI open failed",
                _ => "unknown error",
            }))
        }
    }
}

impl Drop for Pigpio {
    fn drop(&mut self) {
        unsafe {
            pigpio_stop(self.handle);
        }
    }
}

/// A SPI device on a channel.
pub struct Spi {
    pi_handle: i32,
    spi_handle: u32,
}

impl Spi {
    /// Reads data from the SPI device into the given `buf` up to `buf.len()` bytes, and returns
    /// the number of bytes transferred.
    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let size = unsafe {
            spi_read(
                self.pi_handle,
                self.spi_handle,
                buf.as_ptr() as *mut c_char,
                buf.len() as u32,
            )
        };
        if size >= 0 {
            Ok(size as usize)
        } else {
            Err(Self::err(size))
        }
    }

    /// Writes data from `buf` to the SPI device, and returns the number of bytes transferred.
    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        let size = unsafe {
            spi_write(
                self.pi_handle,
                self.spi_handle,
                buf.as_ptr() as *mut c_char,
                buf.len() as u32,
            )
        };
        if size >= 0 {
            Ok(size as usize)
        } else {
            Err(Self::err(size))
        }
    }

    /// Transfers data from `tx_buf` to the SPI device.  Simultaneously data are read from the
    /// device and placed in `rx_buf`. Returns the number of bytes transferred. It checks that
    /// `tx_buf` and `rx_buf` has the same length.
    pub fn xfer(&self, tx_buf: &[u8], rx_buf: &mut [u8]) -> Result<usize> {
        if tx_buf.len() != rx_buf.len() {
            return Err(err_msg("tx_buf and rx_buf must have the same len"));
        }

        let size = unsafe {
            spi_xfer(
                self.pi_handle,
                self.spi_handle,
                tx_buf.as_ptr() as *mut c_char,
                rx_buf.as_ptr() as *mut c_char,
                tx_buf.len() as u32,
            )
        };
        if size >= 0 {
            Ok(size as usize)
        } else {
            Err(Self::err(size))
        }
    }

    fn err(code: i32) -> Box<dyn std::error::Error> {
        err_msg(match code {
            PI_BAD_HANDLE => "bad handle",
            PI_BAD_SPI_COUNT => "bad buffer size",
            PI_SPI_XFER_FAILED => "SPI transfer failed",
            _ => "unknown error",
        })
    }
}

impl Drop for Spi {
    fn drop(&mut self) {
        unsafe {
            spi_close(self.pi_handle, self.spi_handle);
        }
    }
}
