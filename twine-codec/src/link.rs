use typed_builder::TypedBuilder;

use crate::{
    commissioner::SteeringData,
    dataset::{ExtendedPanId, NetworkName},
    radio::PanId,
};

#[derive(Debug, TypedBuilder)]
pub struct ActiveScanResult {
    extended_address: u64,
    network_name: NetworkName,
    xpan_id: ExtendedPanId,
    steering_data: SteeringData,
    pan_id: PanId,
    joiner_udp_port: u16,
    channel: u8,
    rssi: i16,
    lqi: u8,
    version: u8,
    is_native: bool,
    is_joiner: bool,
}

#[derive(Debug, TypedBuilder)]
pub struct EnergyScanResult {
    channel: u8,
    rssi: i8,
}
