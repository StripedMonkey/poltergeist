use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum DualsenseReportType {
    USB = 0x1,
    Bluetooth = 0x31,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum ButtonsMask2 {
    LeftTrigger = 0x1,
    RightTrigger = 0x2,
    LeftBumper = 0x4,
    RightBumper = 0x8,
    Create = 0x10,
    Options = 0x20,
    LeftStickPress = 0x40,
    RightStickPress = 0x80,
}

enum DPadButtons {}

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct Buttons([u8; 4]);

impl Buttons {
    fn mask1_pressed(&self, mask: ButtonsMask2) -> bool {
        (self.0[2] & (mask as u8)) != 0
    }
}

impl Debug for Buttons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buttons")
            .field(
                "left_trigger",
                &self.mask1_pressed(ButtonsMask2::LeftTrigger),
            )
            .field(
                "right_trigger",
                &self.mask1_pressed(ButtonsMask2::RightTrigger),
            )
            .field("left_bumper", &self.mask1_pressed(ButtonsMask2::LeftBumper))
            .field(
                "right_bumper",
                &self.mask1_pressed(ButtonsMask2::RightBumper),
            )
            .field("create", &self.mask1_pressed(ButtonsMask2::Create))
            .field("options", &self.mask1_pressed(ButtonsMask2::Options))
            .field(
                "left_stick_press",
                &self.mask1_pressed(ButtonsMask2::LeftStickPress),
            )
            .field(
                "right_stick_press",
                &self.mask1_pressed(ButtonsMask2::RightStickPress),
            )
            .finish()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct DualsenseInputReport {
    pub sequence_number: u8,
    pub left: XYAxis<u8>,
    pub right: XYAxis<u8>,
    pub triggers: XYAxis<u8>,

    pub buttons: Buttons,
    reserved: [u8; 4],

    pub gyro: XYZAxis<i16>,

    pub accel: XYZAxis<i16>,

    pub sensor_timestamp: u32,
    pub reserved2: [u8; 2],

    pub touchpad: [Touchpoint; 2],

    reserved3: [u8; 11],

    pub battery_state: BatteryState,

    pub peripheral_state: PeripheralState,

    unknown: u8,

    pub aes_cmac: [u8; 8],
}

const _: () = {
    assert!(std::mem::size_of::<Buttons>() == 4);
    assert!(std::mem::size_of::<Touchpoint>() == 4);
    assert!(std::mem::size_of::<DualsenseInputReport>() == 63);

    // These are values I have definitively observed are correct
    assert!(std::mem::offset_of!(DualsenseInputReport, left) == 0x1);
    assert!(std::mem::offset_of!(DualsenseInputReport, right) == 0x3);
    assert!(std::mem::offset_of!(DualsenseInputReport, triggers) == 0x5);
    assert!(std::mem::offset_of!(DualsenseInputReport, touchpad) == 0x21);
};

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Touchpoint {
    contact: u8,

    /// The XY coordinates are 12-bit values packed into 3 bytes
    coordinates: [u8; 3],
}

impl Touchpoint {
    pub fn is_active(&self) -> bool {
        self.contact & 0b1000_0000 == 0
    }

    pub fn touch_count(&self) -> u8 {
        self.contact & 0b0111_1111
    }

    pub fn x(&self) -> u16 {
        let x_low = self.coordinates[0] as u16;
        let x_high = (self.coordinates[1] as u16 & 0x0F) << 8;
        x_low | x_high
    }

    pub fn y(&self) -> u16 {
        let y_low = (self.coordinates[1] as u16 >> 4) & 0x0F;
        let y_high = (self.coordinates[2] as u16) << 4;
        y_low | y_high
    }
}

impl Debug for Touchpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Touchpoint")
            .field("active", &self.is_active())
            .field("touch_count", &self.touch_count())
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct PeripheralState(u8);

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct BatteryState(u8);

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct XYZAxis<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct XYAxis<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ChargingStatus {
    Discharging = 0x0,
    Charging = 0x1,
    Charged = 0x2,
    VoltageOrTemperatureOutOfRange = 0xa,
    TemperatureError = 0xb,
    ChargingError = 0xf,
    Unknown,
}

impl From<u8> for ChargingStatus {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ChargingStatus::Discharging,
            0x1 => ChargingStatus::Charging,
            0x2 => ChargingStatus::Charged,
            0xa => ChargingStatus::VoltageOrTemperatureOutOfRange,
            0xb => ChargingStatus::TemperatureError,
            0xf => ChargingStatus::ChargingError,
            _ => ChargingStatus::Unknown,
        }
    }
}

impl TryFrom<u8> for DualsenseReportType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(DualsenseReportType::USB),
            0x31 => Ok(DualsenseReportType::Bluetooth),
            _ => Err(()),
        }
    }
}
