use std::{ffi::CString, mem::offset_of, os::unix::ffi::OsStrExt};

use clap::Parser as _;
use log::trace;

use crate::{
    constants::hid::MAX_REPORT_SIZE,
    hid::{DualsenseInputReport, DualsenseReportType},
};

mod args;
mod constants;
mod hid;
fn get_dualsense_udev_devices() -> Vec<udev::Device> {
    let mut devices = Vec::new();
    let mut enumerator = udev::Enumerator::new().expect("Failed to create udev enumerator");
    enumerator
        .match_subsystem("input")
        .expect("Failed to match input subsystem");
    enumerator
        .match_subsystem("hid")
        .expect("Failed to match hid subsystem");
    for device in enumerator.scan_devices().expect("Failed to scan devices") {
        if let (Some(vendor_id), Some(product_id)) = (
            device.property_value("ID_VENDOR_ID"),
            device.property_value("ID_MODEL_ID"),
        ) {
            // TODO: Use const_format or similar to avoid a format string at runtime
            if vendor_id.as_bytes()
                == format!("{:04x}", constants::hid::DUALSENSE_VENDOR_ID).as_bytes()
                && product_id.as_bytes()
                    == format!("{:04x}", constants::hid::DUALSENSE_PRODUCT_ID).as_bytes()
            {
                trace!("Found DualSense controller: {:?}", device);
                devices.push(device);
                continue;
            }
        }
        if let Some(hid_id) = device.property_value("HID_ID") {
            if hid_id.as_bytes()
                == format!(
                    "0005:{:08X}:{:08X}",
                    constants::hid::DUALSENSE_VENDOR_ID,
                    constants::hid::DUALSENSE_PRODUCT_ID
                )
                .as_bytes()
            {
                trace!("Found DualSense controller via HID_ID: {:?}", device);
                devices.push(device);
                continue;
            }
        }
    }
    devices
}

fn read_dualsense_report(device: &hidapi::HidDevice) {
    let mut buf = [0u8; MAX_REPORT_SIZE * 2]; // Could be
    match device.read_timeout(&mut buf, 1000) {
        Ok(bytes_read) => {
            trace!("Read {} bytes from DualSense device", bytes_read);
            trace!("{:02x?}", &buf[..bytes_read]);
        }
        Err(e) => {
            panic!("Failed to read from DualSense device: {}", e);
        }
    }
    // Lets get dangerous
    let report_type = DualsenseReportType::try_from(buf[0]);
    let input_report = match report_type {
        Ok(DualsenseReportType::USB) | Ok(DualsenseReportType::Bluetooth) => {
            let input_report: DualsenseInputReport =
                unsafe { std::ptr::read(buf[1..].as_ptr() as *const _) };
            // trace!("Parsed DualSense input report: {:?}", input_report);
            input_report
        }
        Err(_) => {
            panic!("Unknown report type: {}", buf[0]);
        }
    };
    let buf = &buf[1..];

    let left_stick = input_report.left;
    println!("Left Stick: x={}, y={}", left_stick.x, left_stick.y);

    let right_stick = input_report.right;
    println!("Right Stick: x={}, y={}", right_stick.x, right_stick.y);

    let triggers = input_report.triggers;
    println!("Triggers: L={}, R={}", triggers.x, triggers.y);

    let sequence_number = input_report.sequence_number;
    println!("Sequence Number: {}", sequence_number);

    let buttons = input_report.buttons;
    println!("Buttons: {:?}", buttons);

    let gyro = input_report.gyro;
    {
        // For the unaligned access
        let (x, y, z) = (gyro.x, gyro.y, gyro.z);
        println!(
            "Gyro: XYZAxis {{ x: {: >6}, y: {:>6}, z: {:>6} }}",
            x, y, z
        );
    }

    let accel = input_report.accel;
    {
        // For the unaligned access
        let (x, y, z) = (accel.x, accel.y, accel.z);
        println!(
            "Accelerometer: XYZAxis {{ x: {:0>6}, y: {:0>6}, z: {:0>6} }}",
            x, y, z
        );
    }

    let sensor_timestamp = input_report.sensor_timestamp;
    println!("Sensor Timestamp: {}", sensor_timestamp);

    let touchpad = input_report.touchpad;
    for (i, touchpoint) in touchpad.iter().enumerate() {
        println!(
            "Touchpoint: active={: <5}, count={: >3}, x={:04}, y={:04}",
            touchpoint.is_active(),
            touchpoint.touch_count(),
            touchpoint.x(),
            touchpoint.y(),
        );
        print!("Raw contact bits: [ ");
        let base = offset_of!(DualsenseInputReport, touchpad) + (i * std::mem::size_of::<hid::Touchpoint>());
        for i in &buf[base..base + std::mem::size_of::<hid::Touchpoint>()] {
            print!("{:08b} ", i);
        }
        println!("]");
    }

    let battery_state = input_report.battery_state;
    println!("Battery State: {:?}", battery_state);

    let peripheral_state = input_report.peripheral_state;
    println!("Peripheral State: {:?}", peripheral_state);

    println!("Battery State: {:?}", input_report.battery_state);
}

fn init_dualsense_device(device: &udev::Device) -> hidapi::HidDevice {
    trace!(
        "Initializing DualSense device: {:?}",
        device.attribute_value("HID_UNIQ")
    );
    let api = hidapi::HidApi::new().expect("Failed to create HID API instance");
    // Determine if there are any existing hidraw device descriptors for this device
    let hidraw_path = device.syspath().join("hidraw");
    if !hidraw_path.exists() {
        panic!("No hidraw path found for device");
    }
    let device_path = {
        let entries = std::fs::read_dir(hidraw_path)
            .expect("Failed to read hidraw directory")
            .filter_map(|entry| entry.ok())
            .collect::<Vec<_>>();
        match entries.first() {
            Some(entry) => format!("/dev/{}", entry.file_name().to_string_lossy()),
            None => panic!(),
        }
    };
    let device_path = CString::new(device_path).expect("Failed to create CString from device path");
    let device = api.open_path(&device_path).expect("Failed to open device");
    device
}

fn main() {
    env_logger::init();
    let cli = args::Cli::parse();
    trace!("Parsed CLI arguments: {:?}", cli);

    match cli.command {
        Some(command) => match command {
            args::Commands::PowerOff => todo!(),
            args::Commands::Battery => todo!(),
            args::Commands::Info => {
                let dualsense_devices = get_dualsense_udev_devices();
                trace!("DualSense devices: {:?}", dualsense_devices);
                for device in dualsense_devices.iter() {
                    let hid_device = init_dualsense_device(device);
                    let device_info = hid_device.get_device_info().unwrap();
                    println!("Device info: {:?}", device_info);
                    println!("Serial: {:?}", device_info.serial_number());
                }
            }
            args::Commands::Lightbar { state } => todo!(),
            args::Commands::LightbarColor {
                red,
                green,
                blue,
                brightness,
            } => todo!(),
            args::Commands::LedBrightness { number } => todo!(),
            args::Commands::PlayerLeds { number, instant } => todo!(),
            args::Commands::Microphone { state } => todo!(),
            args::Commands::MicrophoneLed { state } => todo!(),
            args::Commands::MicrophoneMode { state } => todo!(),
            args::Commands::MicrophoneVolume { volume } => todo!(),
            args::Commands::Speaker { state } => todo!(),
            args::Commands::Volume { volume } => todo!(),
            args::Commands::Attenuation { rumble, trigger } => todo!(),
            args::Commands::Status => {
                let dualsense_devices = get_dualsense_udev_devices();
                let hid_device = init_dualsense_device(&dualsense_devices[0]);
                loop {
                    read_dualsense_report(&hid_device);
                    // std::thread::sleep(std::time::Duration::from_millis(5));
                }
            }
            args::Commands::Trigger {
                trigger,
                mode,
                params,
            } => todo!(),
            args::Commands::Monitor { action, command } => todo!(),
        },
        None => {
            let dualsense_devices = get_dualsense_udev_devices();
            println!("Found {} DualSense controller(s)", dualsense_devices.len());
        }
    }
}
