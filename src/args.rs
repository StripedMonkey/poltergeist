use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "dualsensectl")]
#[command(about = "Control DualSense controllers", long_about = None)]
pub struct Cli {
    /// List available devices
    #[arg(short, long)]
    pub list: bool,

    /// Specify which device to use
    #[arg(short, long, value_name = "DEVICE")]
    pub device: Option<String>,

    /// Wait for shell command to complete (monitor only)
    #[arg(short, long)]
    pub wait: bool,

    /// Print version info
    #[arg(short, long)]
    pub version: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Turn off the controller (BT only)
    PowerOff,

    /// Get the controller battery level
    Battery,

    /// Get the controller firmware info
    Info,

    /// Enable (on) or disable (off) lightbar
    Lightbar {
        #[arg(value_name = "STATE")]
        state: Option<String>,
    },

    /// Set lightbar color and brightness (0-255)
    LightbarColor {
        #[arg(value_name = "RED")]
        red: u8,
        #[arg(value_name = "GREEN")]
        green: u8,
        #[arg(value_name = "BLUE")]
        blue: u8,
        #[arg(value_name = "BRIGHTNESS")]
        brightness: Option<u8>,
    },

    /// Set player and microphone LED dimming (0-2)
    LedBrightness {
        #[arg(value_name = "NUMBER")]
        number: u8,
    },

    /// Set player LEDs (1-7) or disabled (0)
    PlayerLeds {
        #[arg(value_name = "NUMBER")]
        number: u8,
        #[arg(value_name = "instant")]
        instant: Option<String>,
    },

    /// Enable (on) or disable (off) microphone
    Microphone {
        #[arg(value_name = "STATE")]
        state: String,
    },

    /// Control microphone LED
    MicrophoneLed {
        #[arg(value_name = "STATE")]
        state: String,
    },

    /// Set microphone mode ('chat', 'asr', or 'both')
    MicrophoneMode {
        #[arg(value_name = "STATE")]
        state: String,
    },

    /// Set microphone volume (0-255)
    MicrophoneVolume {
        #[arg(value_name = "VOLUME")]
        volume: u8,
    },

    /// Set speaker output to 'internal', 'headphone', or 'both'
    Speaker {
        #[arg(value_name = "STATE")]
        state: String,
    },

    /// Set audio volume (0-255)
    Volume {
        #[arg(value_name = "VOLUME")]
        volume: u8,
    },

    Status,

    /// Set attenuation (0-7)
    Attenuation {
        #[arg(value_name = "RUMBLE")]
        rumble: u8,
        #[arg(value_name = "TRIGGER")]
        trigger: u8,
    },

    /// Control trigger behavior
    Trigger {
        #[arg(value_name = "TRIGGER")]
        trigger: String,

        #[arg(value_name = "MODE")]
        mode: Option<String>,

        #[arg(value_name = "PARAMS")]
        params: Vec<String>,
    },

    /// Monitor add/remove events and run shell commands
    Monitor {
        #[arg(value_name = "ACTION")]
        action: Option<String>,
        #[arg(value_name = "COMMAND")]
        command: Option<String>,
    },
}
