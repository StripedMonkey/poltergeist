pub mod hid {

    pub const MAX_REPORT_SIZE: usize = 78;

    pub const DUALSENSE_VENDOR_ID: u16 = 0x054C;
    pub const DUALSENSE_PRODUCT_ID: u16 = 0x0CE6;
    pub const DUALSENSE_EDGE_PRODUCT_ID: u16 = 0x0df2;

    pub const TOUCHPAD_WIDTH: u16 = 1920;
    pub const TOUCHPAD_HEIGHT: u16 = 1080;

    pub const ACCELERATION_RESOLUTION_PER_G: u16 = 8192;
    pub const ACCELERATION_RANGE: u16 = 4 * ACCELERATION_RESOLUTION_PER_G;
    
}
