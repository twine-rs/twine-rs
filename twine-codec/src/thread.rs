use core::str::FromStr;

use crate::error::TwineCodecError;

/// The MLE Link Mode configuration.
pub struct LinkModeConfig {
    /// Sender has its receiver on when not transmitting.
    rx_on_when_idle: bool,

    /// The sender is a Full Thread Device.
    device_type: bool,

    /// The sender requires the full Network Data.
    network_data: bool,
}

#[derive(Default)]
pub enum DeviceRole {
    /// Thread networking is disabled
    #[default]
    Disabled,

    /// Not participating in a Thread network
    Detached,

    /// Thread Child role
    Child,

    /// Thread Router role
    Router,

    /// Thread Leader role
    Leader,
}

impl FromStr for DeviceRole {
    type Err = TwineCodecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "disabled" => Ok(DeviceRole::Disabled),
            "detached" => Ok(DeviceRole::Detached),
            "child" => Ok(DeviceRole::Child),
            "router" => Ok(DeviceRole::Router),
            "leader" => Ok(DeviceRole::Leader),
            _ => Err(TwineCodecError::Internal("Unknown DeviceRole")),
        }
    }
}
