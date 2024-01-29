#[derive(Debug, Copy, Clone)]
/// List of SEN5x sensor commands.
/// [Datasheet](https://sensirion.com/media/documents/6791EFA0/62A1F68F/Sensirion_Datasheet_Environmental_Node_SEN5x.pdf) (page 18, ch6.1).
pub enum Command {
    /// Start periodic measurement, signal update interval is 5 seconds.
    StartMeasurement,
    /// Stop periodic measurement and return to idle mode for sensor configuration or to safe energy.
    StopMeasurement,
    /// Is data ready for read-out?
    GetReadDataReadyStatus,
    /// Reading out the serial number can be used to identify the chip and to verify the presence of the sensor.
    GetSerialNumber,
    /// Read sensor output. The measurement data can only be read out once per signal update interval as the buffer is emptied upon read-out.
    ReadMeasurement,
    /// Reinitializes the sensor by reloading user settings from EEPROM.
    Reinit,
    /// Starts the fan-cleaning manually. This command can only be executed in Measurement-Mode.
    StartFanCleaning,
}

impl Command {
    // Command, execution time ms, possibility to execute during measurements.
    pub fn as_tuple(self) -> (u16, u32, bool) {
        match self {
            Self::StartMeasurement => (0x0021, 50, false),
            Self::StopMeasurement => (0x0104, 200, true),
            Self::GetReadDataReadyStatus => (0x0202, 20, true),
            Self::GetSerialNumber => (0xD033, 20, false),
            Self::ReadMeasurement => (0x03C4, 20, true),
            Self::Reinit => (0xD304, 100, false),
            Self::StartFanCleaning => (0x5607, 20, true),
        }
    }
}
