use embedded_hal::{delay::DelayNs, i2c::I2c};

use crate::commands::Command;
use crate::types::Sen5xData;

/// The default I²C address of the SEN5X sensor.
const _SEN5X_I2C_ADDRESS: u8 = 0x69;

/// SEN5x sensor instance. Use related methods to take measurements.
#[derive(Debug, Default)]
pub struct Sen5x<I2C, D> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The concrete Delay implementation.
    delay: D,
    /// Whether the air quality measurement was initialized.
    is_running: bool,
    /// The I2C address of the sensor.
    address: u8,
}

impl<I2C, D> Sen5x<I2C, D>
where
    I2C: I2c,
    D: DelayNs,
{
    /// Create a new instance using the default I2C address.
    pub fn new(i2c: I2C, delay: D) -> Self {
        Self {
            i2c,
            delay,
            is_running: false,
            address: _SEN5X_I2C_ADDRESS,
        }
    }

    /// Create a new instance using a custom I2C address.
    pub fn with_i2c_address(i2c: I2C, delay: D, address: u8) -> Self {
        Self {
            i2c,
            delay,
            is_running: false,
            address,
        }
    }

    /// Start periodic measurement, signal update interval is 1 second.
    pub fn start_measurement(&mut self) -> Result<(), I2C::Error> {
        self.write_command(Command::StartMeasurement)?;
        self.is_running = true;
        Ok(())
    }

    /// The reinit command reinitializes the sensor by reloading user settings from EEPROM.
    pub fn reinit(&mut self) -> Result<(), I2C::Error> {
        self.write_command(Command::Reinit)?;
        Ok(())
    }

    /// Get 48-bit serial number.
    pub fn serial_number(&mut self) -> Result<u64, I2C::Error> {
        let mut buf = [0; 9];
        self.delayed_read_cmd(Command::GetSerialNumber, &mut buf)?;
        let serial = u64::from(buf[0]) << 40
            | u64::from(buf[1]) << 32
            | u64::from(buf[3]) << 24
            | u64::from(buf[4]) << 16
            | u64::from(buf[6]) << 8
            | u64::from(buf[7]);

        Ok(serial)
    }

    /// Read converted sensor data.
    pub fn measurement(&mut self) -> Result<Sen5xData, I2C::Error> {
        let mut buf = [0; 24];
        self.delayed_read_cmd(Command::ReadMeasurement, &mut buf)?;
        // buf[2], buf[5], buf[8], buf[11], buf[14], buf[17], buf[20], buf[23] are CRC bytes and are not used.
        let pm1_0 = u16::from_be_bytes([buf[0], buf[1]]);
        let pm2_5 = u16::from_be_bytes([buf[3], buf[4]]);
        let pm4_0 = u16::from_be_bytes([buf[6], buf[7]]);
        let pm10_0 = u16::from_be_bytes([buf[9], buf[10]]);
        let humidity = u16::from_be_bytes([buf[12], buf[13]]);
        let temperature = u16::from_be_bytes([buf[15], buf[16]]);
        let voc_index = u16::from_be_bytes([buf[18], buf[19]]);
        let nox_index = u16::from_be_bytes([buf[21], buf[22]]);

        Ok(Sen5xData {
            pm1_0: pm1_0 as f32 / 10f32,
            pm2_5: pm2_5 as f32 / 10f32,
            pm4_0: pm4_0 as f32 / 10f32,
            pm10_0: pm10_0 as f32 / 10f32,
            temperature: temperature as f32 / 200f32,
            humidity: humidity as f32 / 100f32,
            voc_index: voc_index as f32 / 10f32,
            nox_index: nox_index as f32 / 10f32,
        })
    }

    /// Check whether new measurement data is available for read-out.
    pub fn data_ready_status(&mut self) -> Result<bool, I2C::Error> {
        let mut buf = [0; 3];
        self.delayed_read_cmd(Command::GetReadDataReadyStatus, &mut buf)?;
        let status = u16::from_be_bytes([buf[0], buf[1]]);

        // 7FF is the last 11 bytes. If they are all zeroes, then data isn't ready.
        let ready = (status & 0x7FF) != 0;
        Ok(ready)
    }

    /// Writes commands without additional arguments.
    fn write_command(&mut self, cmd: Command) -> Result<(), I2C::Error> {
        let (command, delay, _allowed_if_running) = cmd.as_tuple();
        self.write_command_u16(self.address, command)?;
        self.delay.delay_ms(delay);
        Ok(())
    }

    /// Command for reading values from the sensor.
    fn delayed_read_cmd(&mut self, cmd: Command, data: &mut [u8]) -> Result<(), I2C::Error> {
        self.write_command(cmd)?;
        self.read_words_with_crc(self.address, data)?;
        Ok(())
    }

    pub fn write_command_u16(&mut self, address: u8, cmd: u16) -> Result<(), I2C::Error> {
        self.i2c.write(address, &cmd.to_be_bytes())?;
        Ok(())
    }

    // Taken from `sensirion-i2c-rs` (which currently depends on the older version 0.2.x of `embedded-hal`).
    // Remove once `sensirion-i2c-rs` is updated.
    pub fn read_words_with_crc(&mut self, addr: u8, data: &mut [u8]) -> Result<(), I2C::Error> {
        assert!(
            data.len() % 3 == 0,
            "Buffer must hold a multiple of 3 bytes"
        );
        self.i2c.read(addr, data)?;
        // crc8::validate(data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use embedded_hal_mock as hal;

    use self::hal::eh1::delay::NoopDelay as DelayMock;
    use self::hal::eh1::i2c::{Mock as I2cMock, Transaction};
    use super::*;

    /// Test the get_serial_number function
    #[test]
    fn test_get_serial_number() {
        // Arrange
        let (cmd, _, _) = Command::GetSerialNumber.as_tuple();
        let expectations = [
            Transaction::write(_SEN5X_I2C_ADDRESS, cmd.to_be_bytes().to_vec()),
            Transaction::read(
                _SEN5X_I2C_ADDRESS,
                vec![0xbe, 0xef, 0x92, 0xbe, 0xef, 0x92, 0xbe, 0xef, 0x92],
            ),
        ];
        let mut mock = I2cMock::new(&expectations);
        let mut sensor = Sen5x::new(mock.clone(), DelayMock);
        // Act
        let serial = sensor.serial_number().unwrap();
        // Assert
        assert_eq!(serial, 0xbeefbeefbeef);
        mock.done();
    }

    /// Test the measurement function
    #[test]
    fn test_measurement() {
        // Arrange
        let (cmd, _, _) = Command::ReadMeasurement.as_tuple();
        let expectations = [
            Transaction::write(_SEN5X_I2C_ADDRESS, cmd.to_be_bytes().to_vec()),
            Transaction::read(
                _SEN5X_I2C_ADDRESS,
                vec![
                    0x00, 0x12, 0xA0, 0x00, 0x16, 0x64, 0x00, 0x18, 0x7B, 0x00, 0x1A, 0x19, 0x15,
                    0x8A, 0x39, 0x11, 0x81, 0x50, 0x01, 0x68, 0x77, 0x00, 0x0A, 0x5A,
                ],
            ),
        ];
        let mut mock = I2cMock::new(&expectations);
        let mut sensor = Sen5x::new(mock.clone(), DelayMock);
        // Act
        let data = sensor.measurement().unwrap();
        // Assert
        assert_eq!(data.pm2_5, 2.200_f32);
        assert_eq!(data.temperature, 22.405_f32);
        assert_eq!(data.humidity, 55.14_f32);
        mock.done()
    }
}
