// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use anyhow::{Context, Result};
use clap::Parser;
use twine_codec::{
    Channel, ChannelMask, DelayTimer, ExtendedPanId, MeshLocalPrefix, NetworkKey, NetworkName,
    PanId, Pskc,
};

use super::parse;

#[derive(Debug, Parser)]
#[command(about = "Modify an existing operational dataset")]
pub struct Args {
    /// Input dataset as a TLV hex string (or pipe via stdin)
    pub dataset: Option<String>,

    /// Set the active timestamp (format: seconds:ticks:authoritative = 1:0:false)
    #[arg(long)]
    pub active_timestamp: Option<String>,

    /// Set the pending timestamp (format: seconds:ticks:authoritative = 1:0:false)
    #[arg(long)]
    pub pending_timestamp: Option<String>,

    /// Set the channel number (page 0)
    #[arg(long)]
    pub channel: Option<u16>,

    /// Set the channel mask (hex = 0x07FFF800)
    #[arg(long)]
    pub channel_mask: Option<String>,

    /// Set the extended PAN ID (hex, 16 chars)
    #[arg(long)]
    pub extended_pan_id: Option<String>,

    /// Set the mesh-local prefix (hex, 16 chars)
    #[arg(long)]
    pub mesh_local_prefix: Option<String>,

    /// Set the network key (hex, 32 chars)
    #[arg(long)]
    pub network_key: Option<String>,

    /// Set the network name (UTF-8, max 16 chars)
    #[arg(long)]
    pub network_name: Option<String>,

    /// Set the PAN ID (hex = 0xb3de)
    #[arg(long)]
    pub pan_id: Option<String>,

    /// Set the PSKc (hex, 32 chars)
    #[arg(long)]
    pub pskc: Option<String>,

    /// Set the delay timer in milliseconds
    #[arg(long)]
    pub delay_timer: Option<u32>,

    /// Set the security policy rotation time in hours
    #[arg(long)]
    pub sp_rotation_time: Option<u16>,

    /// Set the security policy obtain-network-key flag
    #[arg(long)]
    pub sp_obtain_network_key: Option<bool>,

    /// Set the security policy native-commissioning flag
    #[arg(long)]
    pub sp_native_commissioning: Option<bool>,

    /// Set the security policy legacy-routers flag
    #[arg(long)]
    pub sp_legacy_routers: Option<bool>,

    /// Set the security policy external-commissioner flag
    #[arg(long)]
    pub sp_external_commissioner: Option<bool>,

    /// Set the security policy commercial-commissioning flag
    #[arg(long)]
    pub sp_commercial_commissioning: Option<bool>,

    /// Set the security policy autonomous-enrollment flag
    #[arg(long)]
    pub sp_autonomous_enrollment: Option<bool>,

    /// Set the security policy network-key-provisioning flag
    #[arg(long)]
    pub sp_network_key_provisioning: Option<bool>,

    /// Set the security policy to-BLE-link flag
    #[arg(long)]
    pub sp_to_ble_link: Option<bool>,

    /// Set the security policy non-CCM-routers flag
    #[arg(long)]
    pub sp_non_ccm_routers: Option<bool>,

    /// Remove the active timestamp
    #[arg(long)]
    pub rm_active_timestamp: bool,

    /// Remove the pending timestamp
    #[arg(long)]
    pub rm_pending_timestamp: bool,

    /// Remove the channel
    #[arg(long)]
    pub rm_channel: bool,

    /// Remove the channel mask
    #[arg(long)]
    pub rm_channel_mask: bool,

    /// Remove the extended PAN ID
    #[arg(long)]
    pub rm_extended_pan_id: bool,

    /// Remove the mesh-local prefix
    #[arg(long)]
    pub rm_mesh_local_prefix: bool,

    /// Remove the network key
    #[arg(long)]
    pub rm_network_key: bool,

    /// Remove the network name
    #[arg(long)]
    pub rm_network_name: bool,

    /// Remove the PAN ID
    #[arg(long)]
    pub rm_pan_id: bool,

    /// Remove the PSKc
    #[arg(long)]
    pub rm_pskc: bool,

    /// Remove the security policy
    #[arg(long)]
    pub rm_security_policy: bool,

    /// Remove the delay timer
    #[arg(long)]
    pub rm_delay_timer: bool,
}

fn has_security_policy_flags(args: &Args) -> bool {
    args.sp_rotation_time.is_some()
        || args.sp_obtain_network_key.is_some()
        || args.sp_native_commissioning.is_some()
        || args.sp_legacy_routers.is_some()
        || args.sp_external_commissioner.is_some()
        || args.sp_commercial_commissioning.is_some()
        || args.sp_autonomous_enrollment.is_some()
        || args.sp_network_key_provisioning.is_some()
        || args.sp_to_ble_link.is_some()
        || args.sp_non_ccm_routers.is_some()
}

fn apply(
    args: Args,
    mut dataset: twine_codec::OperationalDataset,
) -> Result<twine_codec::OperationalDataset> {
    // Apply removals first
    if args.rm_active_timestamp {
        dataset.remove_active_timestamp();
    }
    if args.rm_pending_timestamp {
        dataset.remove_pending_timestamp();
    }
    if args.rm_channel {
        dataset.remove_channel();
    }
    if args.rm_channel_mask {
        dataset.remove_channel_mask();
    }
    if args.rm_extended_pan_id {
        dataset.remove_extended_pan_id();
    }
    if args.rm_mesh_local_prefix {
        dataset.remove_mesh_local_prefix();
    }
    if args.rm_network_key {
        dataset.remove_network_key();
    }
    if args.rm_network_name {
        dataset.remove_network_name();
    }
    if args.rm_pan_id {
        dataset.remove_pan_id();
    }
    if args.rm_pskc {
        dataset.remove_pskc();
    }
    if args.rm_security_policy {
        dataset.remove_security_policy();
    }
    if args.rm_delay_timer {
        dataset.remove_delay_timer();
    }

    // Apply set operations
    if let Some(ref ts) = args.active_timestamp {
        let timestamp = parse::timestamp(ts).context("--active-timestamp")?;
        dataset
            .set_active_timestamp(timestamp)
            .context("failed to set active timestamp")?;
    }

    if let Some(ref ts) = args.pending_timestamp {
        let timestamp = parse::timestamp(ts).context("--pending-timestamp")?;
        dataset
            .set_pending_timestamp(timestamp)
            .context("failed to set pending timestamp")?;
    }

    if let Some(channel) = args.channel {
        dataset
            .set_channel(Channel::new(0, channel))
            .context("failed to set channel")?;
    }

    if let Some(ref mask) = args.channel_mask {
        let channel_mask = ChannelMask::from_str(mask).context("failed to parse --channel-mask")?;
        dataset
            .set_channel_mask(channel_mask)
            .context("failed to set channel mask")?;
    }

    if let Some(ref hex) = args.extended_pan_id {
        let bytes = parse::hex_bytes::<8>(hex, "extended-pan-id")?;
        dataset
            .set_extended_pan_id(ExtendedPanId::from(bytes))
            .context("failed to set extended PAN ID")?;
    }

    if let Some(ref hex) = args.mesh_local_prefix {
        let bytes = parse::hex_bytes::<8>(hex, "mesh-local-prefix")?;
        dataset
            .set_mesh_local_prefix(MeshLocalPrefix::from(bytes))
            .context("failed to set mesh-local prefix")?;
    }

    if let Some(ref hex) = args.network_key {
        let key = NetworkKey::from_str(hex).context("failed to parse --network-key")?;
        dataset
            .set_network_key(key)
            .context("failed to set network key")?;
    }

    if let Some(ref name) = args.network_name {
        let nn = NetworkName::from_str(name).context("failed to parse --network-name")?;
        dataset
            .set_network_name(nn)
            .context("failed to set network name")?;
    }

    if let Some(ref hex) = args.pan_id {
        let pan_id = PanId::from_str(hex).context("failed to parse --pan-id")?;
        dataset.set_pan_id(pan_id).context("failed to set PAN ID")?;
    }

    if let Some(ref hex) = args.pskc {
        let bytes = parse::hex_bytes::<16>(hex, "pskc")?;
        dataset
            .set_pskc(Pskc::from(bytes))
            .context("failed to set PSKc")?;
    }

    if let Some(millis) = args.delay_timer {
        dataset
            .set_delay_timer(DelayTimer::from(millis))
            .context("failed to set delay timer")?;
    }

    // Security policy: modify in-place if any sp flag is set
    if has_security_policy_flags(&args) {
        let mut policy = dataset.security_policy().unwrap_or_default();

        if let Some(hours) = args.sp_rotation_time {
            policy.set_rotation_time_hours(hours);
        }
        if let Some(v) = args.sp_obtain_network_key {
            policy.set_obtain_network_key(v);
        }
        if let Some(v) = args.sp_native_commissioning {
            policy.set_native_commissioning(v);
        }
        if let Some(v) = args.sp_legacy_routers {
            policy.set_legacy_routers(v);
        }
        if let Some(v) = args.sp_external_commissioner {
            policy.set_external_commissioner(v);
        }
        if let Some(v) = args.sp_commercial_commissioning {
            policy.set_commercial_commissioning(v);
        }
        if let Some(v) = args.sp_autonomous_enrollment {
            policy.set_autonomous_enrollment(v);
        }
        if let Some(v) = args.sp_network_key_provisioning {
            policy.set_network_key_provisioning(v);
        }
        if let Some(v) = args.sp_to_ble_link {
            policy.set_to_ble_link(v);
        }
        if let Some(v) = args.sp_non_ccm_routers {
            policy.set_non_ccm_routers(v);
        }

        dataset
            .set_security_policy(policy)
            .context("failed to set security policy")?;
    }

    Ok(dataset)
}

pub fn run(args: Args) -> Result<()> {
    let dataset = parse::dataset_from_arg_or_stdin(args.dataset.as_deref())?;
    let dataset = apply(args, dataset)?;
    println!("{}", dataset.as_hex_string());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use twine_codec::OperationalDataset;

    const DATASET_HEX: &str = "0e080000000000010000000300000c4a0300001335060004001fffe002081bb896bef533a5850708fd48b2e8c34e7dc70510e9b948988752752873570d09ada4d0be030f4f70656e5468726561642d623364650102b3de0410f9f07ed37fbb6828fb3b26b63bdea3c30c0402a0f7f8";

    fn base_args() -> Args {
        Args {
            dataset: Some(DATASET_HEX.to_string()),
            active_timestamp: None,
            pending_timestamp: None,
            channel: None,
            channel_mask: None,
            extended_pan_id: None,
            mesh_local_prefix: None,
            network_key: None,
            network_name: None,
            pan_id: None,
            pskc: None,
            delay_timer: None,
            sp_rotation_time: None,
            sp_obtain_network_key: None,
            sp_native_commissioning: None,
            sp_legacy_routers: None,
            sp_external_commissioner: None,
            sp_commercial_commissioning: None,
            sp_autonomous_enrollment: None,
            sp_network_key_provisioning: None,
            sp_to_ble_link: None,
            sp_non_ccm_routers: None,
            rm_active_timestamp: false,
            rm_pending_timestamp: false,
            rm_channel: false,
            rm_channel_mask: false,
            rm_extended_pan_id: false,
            rm_mesh_local_prefix: false,
            rm_network_key: false,
            rm_network_name: false,
            rm_pan_id: false,
            rm_pskc: false,
            rm_security_policy: false,
            rm_delay_timer: false,
        }
    }

    fn dataset() -> OperationalDataset {
        parse::dataset(DATASET_HEX).unwrap()
    }

    #[test]
    fn no_ops_roundtrip() {
        let result = apply(base_args(), dataset()).unwrap();
        assert_eq!(result.as_hex_string(), DATASET_HEX);
    }

    #[test]
    fn set_channel() {
        let args = Args {
            channel: Some(25),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(result.channel().unwrap().channel(), 25);
    }

    #[test]
    fn set_network_name() {
        let args = Args {
            network_name: Some("TestNet".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(result.network_name().unwrap().as_str(), "TestNet");
    }

    #[test]
    fn set_pan_id_with_prefix() {
        let args = Args {
            pan_id: Some("0xDEAD".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(result.pan_id().unwrap().to_string(), "0xdead");
    }

    #[test]
    fn set_pan_id_without_prefix() {
        let args = Args {
            pan_id: Some("dead".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(result.pan_id().unwrap().to_string(), "0xdead");
    }

    #[test]
    fn set_active_timestamp() {
        let args = Args {
            active_timestamp: Some("42:0:false".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(result.active_timestamp().unwrap().seconds(), 42);
        assert_eq!(result.active_timestamp().unwrap().ticks(), 0);
        assert!(!result.active_timestamp().unwrap().is_authoritative());
    }

    #[test]
    fn set_delay_timer() {
        let args = Args {
            delay_timer: Some(30_000),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(
            result.delay_timer().unwrap().duration(),
            core::time::Duration::from_millis(30_000)
        );
    }

    #[test]
    fn set_extended_pan_id_without_prefix() {
        let args = Args {
            extended_pan_id: Some("aabbccddeeff0011".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(
            result.extended_pan_id().unwrap().as_bytes(),
            &[0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11]
        );
    }

    #[test]
    fn set_extended_pan_id_with_prefix() {
        let args = Args {
            extended_pan_id: Some("0xaabbccddeeff0011".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(
            result.extended_pan_id().unwrap().as_bytes(),
            &[0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11]
        );
    }

    #[test]
    fn set_mesh_local_prefix_without_prefix() {
        let args = Args {
            mesh_local_prefix: Some("fd001234abcd5678".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(
            result.mesh_local_prefix().unwrap().to_string(),
            "fd00:1234:abcd:5678::/64"
        );
    }

    #[test]
    fn set_mesh_local_prefix_with_prefix() {
        let args = Args {
            mesh_local_prefix: Some("0xfd001234abcd5678".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(
            result.mesh_local_prefix().unwrap().to_string(),
            "fd00:1234:abcd:5678::/64"
        );
    }

    #[test]
    fn set_network_key_without_prefix() {
        let key_hex = "00112233445566778899aabbccddeeff";
        let args = Args {
            network_key: Some(key_hex.to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(result.network_key().unwrap().to_string(), key_hex);
    }

    #[test]
    fn set_network_key_with_prefix() {
        let args = Args {
            network_key: Some("0x00112233445566778899aabbccddeeff".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(
            result.network_key().unwrap().to_string(),
            "00112233445566778899aabbccddeeff"
        );
    }

    #[test]
    fn set_pskc_without_prefix() {
        let args = Args {
            pskc: Some("aabbccddeeff00112233445566778899".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(
            result.pskc().unwrap().to_string(),
            "aabbccddeeff00112233445566778899"
        );
    }

    #[test]
    fn set_pskc_with_prefix() {
        let args = Args {
            pskc: Some("0xaabbccddeeff00112233445566778899".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(
            result.pskc().unwrap().to_string(),
            "aabbccddeeff00112233445566778899"
        );
    }

    #[test]
    fn set_channel_mask_without_prefix() {
        let args = Args {
            channel_mask: Some("07FFF800".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(result.channel_mask().unwrap().to_string(), "0x07fff800");
    }

    #[test]
    fn set_channel_mask_with_prefix() {
        let args = Args {
            channel_mask: Some("0x07FFF800".to_string()),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(result.channel_mask().unwrap().to_string(), "0x07fff800");
    }

    #[test]
    fn set_security_policy_rotation_time() {
        let args = Args {
            sp_rotation_time: Some(100),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(result.security_policy().unwrap().rotation_time_hours(), 100);
    }

    #[test]
    fn set_security_policy_flags() {
        let args = Args {
            sp_obtain_network_key: Some(false),
            sp_native_commissioning: Some(false),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        let policy = result.security_policy().unwrap();
        assert!(!policy.obtain_network_key_enabled());
        assert!(!policy.native_commissioning_enabled());
    }

    #[test]
    fn remove_active_timestamp() {
        let args = Args {
            rm_active_timestamp: true,
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert!(result.active_timestamp().is_none());
    }

    #[test]
    fn remove_pending_timestamp() {
        // Base dataset has no pending timestamp, so set one first then remove it.
        let with_pending = apply(
            Args {
                pending_timestamp: Some("1:0:false".to_string()),
                ..base_args()
            },
            dataset(),
        )
        .unwrap();
        assert!(with_pending.pending_timestamp().is_some());

        let result = apply(
            Args {
                rm_pending_timestamp: true,
                ..base_args()
            },
            with_pending,
        )
        .unwrap();
        assert!(result.pending_timestamp().is_none());
    }

    #[test]
    fn remove_channel() {
        let args = Args {
            rm_channel: true,
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert!(result.channel().is_none());
    }

    #[test]
    fn remove_channel_mask() {
        let args = Args {
            rm_channel_mask: true,
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert!(result.channel_mask().is_none());
    }

    #[test]
    fn remove_extended_pan_id() {
        let args = Args {
            rm_extended_pan_id: true,
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert!(result.extended_pan_id().is_none());
    }

    #[test]
    fn remove_mesh_local_prefix() {
        let args = Args {
            rm_mesh_local_prefix: true,
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert!(result.mesh_local_prefix().is_none());
    }

    #[test]
    fn remove_network_key() {
        let args = Args {
            rm_network_key: true,
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert!(result.network_key().is_none());
    }

    #[test]
    fn remove_network_name() {
        let args = Args {
            rm_network_name: true,
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert!(result.network_name().is_none());
    }

    #[test]
    fn remove_pan_id() {
        let args = Args {
            rm_pan_id: true,
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert!(result.pan_id().is_none());
    }

    #[test]
    fn remove_pskc() {
        let args = Args {
            rm_pskc: true,
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert!(result.pskc().is_none());
    }

    #[test]
    fn remove_security_policy() {
        let args = Args {
            rm_security_policy: true,
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert!(result.security_policy().is_none());
    }

    #[test]
    fn remove_delay_timer() {
        // Base dataset has no delay timer, so set one first then remove it.
        let with_timer = apply(
            Args {
                delay_timer: Some(30_000),
                ..base_args()
            },
            dataset(),
        )
        .unwrap();
        assert!(with_timer.delay_timer().is_some());

        let result = apply(
            Args {
                rm_delay_timer: true,
                ..base_args()
            },
            with_timer,
        )
        .unwrap();
        assert!(result.delay_timer().is_none());
    }

    #[test]
    fn remove_then_set_channel() {
        // Remove and set in one call, set should still apply correctly because removals are applied first
        let args = Args {
            rm_channel: true,
            channel: Some(20),
            ..base_args()
        };
        let result = apply(args, dataset()).unwrap();
        assert_eq!(result.channel().unwrap().channel(), 20);
    }

    #[test]
    fn invalid_timestamp_format() {
        let args = Args {
            active_timestamp: Some("bad".to_string()),
            ..base_args()
        };
        assert!(apply(args, dataset()).is_err());
    }

    #[test]
    fn invalid_hex_bytes_length() {
        let args = Args {
            extended_pan_id: Some("aabb".to_string()),
            ..base_args()
        };
        assert!(apply(args, dataset()).is_err());
    }
}
