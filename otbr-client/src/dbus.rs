use std::{str::FromStr, time::Duration};

use async_trait::async_trait;
use twine_codec::{
    commissioner::{JoinerPskd, SteeringData},
    dataset::{
        ExtendedPanId, NetworkKey, NetworkName, OperationalDataset, OperationalDatasetTlvs, Pskc,
    },
    link::{ActiveScanResult, EnergyScanResult},
    radio::{Channel, ChannelMask, PanId},
    thread::DeviceRole,
};
use zbus::{proxy, Connection, Result};

use super::{error::OtbrClientError, OtbrClient, OtbrClientResult};

/// Format of the active scan result entry over D-Bus.
type DbusActiveScanResultEntry = (
    u64,
    String,
    u64,
    Vec<u8>,
    u16,
    u16,
    u8,
    i16,
    u8,
    u8,
    bool,
    bool,
);

/// Format of the energy scan result entry over D-Bus.
type DbusEnergyScanEntry = (u8, u8);

/// Format of the MAC counters over D-Bus.
type DbusMacCounters = (
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
);

pub struct OtbrDbusClient<'p> {
    proxy: BorderRouterProxy<'p>,
}

impl<'p> OtbrDbusClient<'p> {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system().await?;

        let proxy = BorderRouterProxy::builder(&connection)
            .cache_properties(proxy::CacheProperties::No)
            .build()
            .await?;

        Ok(Self { proxy })
    }
}

#[async_trait]
impl<'p> OtbrClient for OtbrDbusClient<'p> {
    async fn active_dataset_tlvs(&self) -> OtbrClientResult<OperationalDatasetTlvs> {
        Ok(self.proxy.active_dataset_tlvs().await?.try_into()?)
    }

    async fn attach(
        &self,
        key: Option<NetworkKey>,
        pan: Option<PanId>,
        name: NetworkName,
        xpan: Option<ExtendedPanId>,
        pskc: Option<Pskc>,
        channel_mask: ChannelMask,
    ) -> OtbrClientResult<()> {
        let key = match key {
            Some(k) => k.into(),
            None => Vec::<u8>::new(),
        };

        let pan = match pan {
            Some(p) => p.into(),
            None => u16::MAX,
        };

        let xpan = match xpan {
            Some(x) => x.into(),
            None => u64::MAX,
        };

        let pskc = match pskc {
            Some(p) => p.into(),
            None => u128::MAX,
        }
        .to_be_bytes();

        let name: String = name.to_string();

        let channel_mask = channel_mask.mask();

        self.proxy
            .attach(&key, pan, &name, xpan, &pskc, channel_mask)
            .await?;

        Ok(())
    }

    async fn attach_all_nodes_to(
        &self,
        dataset: OperationalDatasetTlvs,
    ) -> OtbrClientResult<Option<Duration>> {
        todo!()
    }

    async fn detach(&self) -> OtbrClientResult<()> {
        self.proxy.detach().await?;
        Ok(())
    }

    async fn device_role(&self) -> OtbrClientResult<DeviceRole> {
        let role = self.proxy.device_role().await?;
        Ok(DeviceRole::from_str(&role)?)
    }

    async fn energy_scan(&self, duration: Duration) -> OtbrClientResult<Vec<EnergyScanResult>> {
        let millis = duration.as_millis() as u32;
        let result = self.proxy.energy_scan(millis).await?;

        Ok(result
            .into_iter()
            .map(|item| {
                EnergyScanResult::builder()
                    .channel(item.0)
                    .rssi(item.1 as i8)
                    .build()
            })
            .collect())
    }

    async fn factory_reset(&self) -> OtbrClientResult<()> {
        self.proxy.factory_reset().await?;
        Ok(())
    }

    // async fn joiner_start(
    //     &self,
    //     pskd: JoinerPskd,
    //     url: ProvisioningUrl,
    //     vendor_info: ThreadVendorInfo,
    // ) -> OtbrClientResult<()> {
    //     todo!()
    // }

    async fn joiner_stop(&self) -> OtbrClientResult<()> {
        self.proxy.joiner_stop().await?;
        Ok(())
    }

    async fn leave_network(&self) -> OtbrClientResult<()> {
        self.proxy.leave_network().await?;
        Ok(())
    }

    async fn permit_unsecure_join(&self, port: u16, duration: Duration) -> OtbrClientResult<()> {
        self.proxy
            .permit_unsecure_join(port, duration.as_millis() as u32)
            .await?;
        Ok(())
    }

    async fn reset(&self) -> OtbrClientResult<()> {
        self.proxy.reset().await?;
        Ok(())
    }

    async fn scan(&self) -> OtbrClientResult<Vec<ActiveScanResult>> {
        self.proxy
            .scan()
            .await?
            .into_iter()
            .map(|item| {
                let network_name = NetworkName::from_str(&item.1)?;
                let steering_data = item.3.try_into()?;

                Ok(ActiveScanResult::builder()
                    .extended_address(item.0)
                    .network_name(network_name)
                    .xpan_id(item.2.into())
                    .steering_data(steering_data)
                    .pan_id(item.4.into())
                    .joiner_udp_port(item.5)
                    .channel(item.6)
                    .rssi(item.7)
                    .lqi(item.8)
                    .version(item.9)
                    .is_native(item.10)
                    .is_joiner(item.11)
                    .build())
            })
            .collect()
    }
}

#[proxy(
    interface = "io.openthread.BorderRouter",
    default_service = "io.openthread.BorderRouter.wpan0",
    default_path = "/io/openthread/BorderRouter/wpan0"
)]
pub trait BorderRouter {
    fn scan(&self) -> zbus::Result<Vec<DbusActiveScanResultEntry>>;

    fn energy_scan(&self, scanduration: u32) -> zbus::Result<Vec<DbusEnergyScanEntry>>;

    fn attach(
        &self,
        networkkey: &[u8],
        panid: u16,
        networkname: &str,
        extpanid: u64,
        pskc: &[u8],
        channel_mask: u32,
    ) -> zbus::Result<()>;

    fn attach_all_nodes_to(&self, dataset: &[u8]) -> zbus::Result<i64>;

    fn detach(&self) -> zbus::Result<()>;

    fn add_external_route(&self, prefix: &(&(&[u8], u8), u16, u8, bool, bool)) -> zbus::Result<()>;

    /// AddOnMeshPrefix method
    fn add_on_mesh_prefix(
        &self,
        prefix: &(
            &(&[u8], u8),
            u16,
            u8,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
        ),
    ) -> zbus::Result<()>;

    /// FactoryReset method
    fn factory_reset(&self) -> zbus::Result<()>;

    /// GetProperties method
    fn get_properties(&self, properties: &[&str]) -> zbus::Result<()>;

    /// JoinerStart method
    fn joiner_start(
        &self,
        pskd: &str,
        provision_url: &str,
        vendor_name: &str,
        vendor_model: &str,
        vendor_sw_version: &str,
        vendor_data: &str,
    ) -> zbus::Result<()>;

    /// JoinerStop method
    fn joiner_stop(&self) -> zbus::Result<()>;

    /// LeaveNetwork method
    fn leave_network(&self) -> zbus::Result<()>;

    /// PermitUnsecureJoin method
    fn permit_unsecure_join(&self, port: u16, timeout: u32) -> zbus::Result<()>;

    /// RemoveExternalRoute method
    fn remove_external_route(&self, prefix: &(&[u8], u8)) -> zbus::Result<()>;

    /// RemoveOnMeshPrefix method
    fn remove_on_mesh_prefix(&self, prefix: &(&[u8], u8)) -> zbus::Result<()>;

    /// Reset method
    fn reset(&self) -> zbus::Result<()>;

    /// SetNat64Enabled method
    fn set_nat64_enabled(&self, enable: bool) -> zbus::Result<()>;

    /// UpdateVendorMeshCopTxtEntries method
    fn update_vendor_mesh_cop_txt_entries(&self, update: &[&(&str, &[u8])]) -> zbus::Result<()>;

    /// Ready signal
    #[zbus(signal)]
    fn ready(&self) -> zbus::Result<()>;

    /// ActiveDatasetTlvs property
    #[zbus(property)]
    fn active_dataset_tlvs(&self) -> zbus::Result<Vec<u8>>;
    #[zbus(property)]
    fn set_active_dataset_tlvs(&self, value: &[u8]) -> zbus::Result<()>;

    /// BorderRoutingCounters property
    #[zbus(property)]
    fn border_routing_counters(
        &self,
    ) -> zbus::Result<(
        (u64, u64),
        (u64, u64),
        (u64, u64),
        (u64, u64),
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
    )>;

    /// Capabilities property
    #[zbus(property)]
    fn capabilities(&self) -> zbus::Result<Vec<u8>>;

    /// CcaFailureRate property
    #[zbus(property)]
    fn cca_failure_rate(&self) -> zbus::Result<u16>;

    /// Channel property
    #[zbus(property)]
    fn channel(&self) -> zbus::Result<u16>;

    /// ChannelMonitorChannelQualityMap property
    #[zbus(property)]
    fn channel_monitor_channel_quality_map(&self) -> zbus::Result<Vec<(u8, u16)>>;

    /// ChannelMonitorSampleCount property
    #[zbus(property)]
    fn channel_monitor_sample_count(&self) -> zbus::Result<u32>;

    /// ChildTable property
    #[zbus(property)]
    fn child_table(
        &self,
    ) -> zbus::Result<
        Vec<(
            u64,
            u32,
            u32,
            u16,
            u16,
            u8,
            u8,
            u8,
            u8,
            u16,
            u16,
            bool,
            bool,
            bool,
            bool,
        )>,
    >;

    /// DeviceRole property
    #[zbus(property)]
    fn device_role(&self) -> zbus::Result<String>;

    /// DnsUpstreamQueryState property
    #[zbus(property)]
    fn dns_upstream_query_state(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_dns_upstream_query_state(&self, value: bool) -> zbus::Result<()>;

    /// DnssdCounters property
    #[zbus(property)]
    fn dnssd_counters(&self) -> zbus::Result<(u32, u32, u32, u32, u32, u32, u32)>;

    /// Eui64 property
    #[zbus(property)]
    fn eui64(&self) -> zbus::Result<u64>;

    /// ExtPanId property
    #[zbus(property)]
    fn ext_pan_id(&self) -> zbus::Result<u64>;

    /// ExtendedAddress property
    #[zbus(property)]
    fn extended_address(&self) -> zbus::Result<u64>;

    /// ExternalRoutes property
    #[zbus(property)]
    fn external_routes(&self) -> zbus::Result<((Vec<u8>, u8), u16, u8, bool, bool)>;

    /// FeatureFlagListData property
    #[zbus(property)]
    fn feature_flag_list_data(&self) -> zbus::Result<Vec<u8>>;
    #[zbus(property)]
    fn set_feature_flag_list_data(&self, value: &[u8]) -> zbus::Result<()>;

    /// InfraLinkInfo property
    #[zbus(property)]
    fn infra_link_info(&self) -> zbus::Result<(String, bool, bool, bool, u32, u32, u32)>;

    /// InstantRssi property
    #[zbus(property)]
    fn instant_rssi(&self) -> zbus::Result<u8>;

    /// LeaderData property
    #[zbus(property)]
    fn leader_data(&self) -> zbus::Result<(u32, u8, u8, u8, u8)>;

    /// LinkCounters property
    #[zbus(property)]
    fn link_counters(&self) -> zbus::Result<(u32, u32, u32, u32)>;

    /// LinkMode property
    #[zbus(property)]
    fn link_mode(&self) -> zbus::Result<(bool, bool, bool)>;
    // #[zbus(property)]
    // fn set_link_mode(&self, value: &(bool, bool, bool)) -> zbus::Result<()>;

    /// LinkPreferredChannelMask property
    #[zbus(property)]
    fn link_preferred_channel_mask(&self) -> zbus::Result<u32>;

    /// LinkSupportedChannelMask property
    #[zbus(property)]
    fn link_supported_channel_mask(&self) -> zbus::Result<u32>;

    /// LocalLeaderWeight property
    #[zbus(property)]
    fn local_leader_weight(&self) -> zbus::Result<u8>;

    /// MacCounters property
    // #[zbus(property)]
    // fn mac_counters(
    //     &self,
    // ) -> zbus::Result<DbusMacCounters>;

    /// MdnsTelemetryInfo property
    #[zbus(property)]
    #[allow(clippy::type_complexity)]
    fn mdns_telemetry_info(
        &self,
    ) -> zbus::Result<(
        (u32, u32, u32, u32, u32, u32, u32, u32),
        (u32, u32, u32, u32, u32, u32, u32, u32),
        (u32, u32, u32, u32, u32, u32, u32, u32),
        (u32, u32, u32, u32, u32, u32, u32, u32),
        u32,
        u32,
        u32,
        u32,
    )>;

    /// MeshLocalPrefix property
    #[zbus(property)]
    fn mesh_local_prefix(&self) -> zbus::Result<Vec<u8>>;
    #[zbus(property)]
    fn set_mesh_local_prefix(&self, value: &[u8]) -> zbus::Result<()>;

    /// Nat64Cidr property
    #[zbus(property)]
    fn nat64_cidr(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_nat64_cidr(&self, value: &str) -> zbus::Result<()>;

    /// Nat64ErrorCounters property
    #[zbus(property)]
    fn nat64_error_counters(
        &self,
    ) -> zbus::Result<((u64, u64), (u64, u64), (u64, u64), (u64, u64))>;

    /// Nat64Mappings property
    #[zbus(property)]
    #[allow(clippy::type_complexity)]
    fn nat64_mappings(
        &self,
    ) -> zbus::Result<
        Vec<(
            u64,
            Vec<u8>,
            Vec<u8>,
            u32,
            (
                (u64, u64, u64, u64),
                (u64, u64, u64, u64),
                (u64, u64, u64, u64),
                (u64, u64, u64, u64),
            ),
        )>,
    >;

    /// Nat64ProtocolCounters property
    #[zbus(property)]
    fn nat64_protocol_counters(
        &self,
    ) -> zbus::Result<(
        (u64, u64, u64, u64),
        (u64, u64, u64, u64),
        (u64, u64, u64, u64),
        (u64, u64, u64, u64),
    )>;

    /// Nat64State property
    #[zbus(property)]
    fn nat64_state(&self) -> zbus::Result<(String, String)>;

    /// NeighborTable property
    #[zbus(property)]
    fn neighbor_table(
        &self,
    ) -> zbus::Result<
        Vec<(
            u64,
            u32,
            u16,
            u32,
            u32,
            u8,
            u8,
            u8,
            u16,
            u16,
            u16,
            bool,
            bool,
            bool,
            bool,
        )>,
    >;

    /// NetworkData property
    #[zbus(property)]
    fn network_data(&self) -> zbus::Result<Vec<u8>>;

    /// NetworkName property
    #[zbus(property)]
    fn network_name(&self) -> zbus::Result<String>;

    /// OnMeshPrefixes property
    #[zbus(property)]
    #[allow(clippy::type_complexity)]
    fn on_mesh_prefixes(
        &self,
    ) -> zbus::Result<
        Vec<(
            (Vec<u8>, u8),
            u16,
            u8,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
        )>,
    >;

    /// OtHostVersion property
    #[zbus(property)]
    fn ot_host_version(&self) -> zbus::Result<String>;

    /// OtRcpVersion property
    #[zbus(property)]
    fn ot_rcp_version(&self) -> zbus::Result<String>;

    /// OtbrVersion property
    #[zbus(property)]
    fn otbr_version(&self) -> zbus::Result<String>;

    /// PanId property
    #[zbus(property)]
    fn pan_id(&self) -> zbus::Result<u16>;

    /// PartitionId property
    #[zbus(property)]
    fn partition_id(&self) -> zbus::Result<u32>;

    /// PendingDatasetTlvs property
    #[zbus(property)]
    fn pending_dataset_tlvs(&self) -> zbus::Result<Vec<u8>>;

    /// RadioCoexMetrics property
    // #[zbus(property)]
    // fn radio_coex_metrics(
    //     &self,
    // ) -> zbus::Result<(
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     bool,
    // )>;

    /// RadioRegion property
    #[zbus(property)]
    fn radio_region(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_radio_region(&self, value: &str) -> zbus::Result<()>;

    /// RadioSpinelMetrics property
    #[zbus(property)]
    fn radio_spinel_metrics(&self) -> zbus::Result<(u32, u32, u32, u32)>;

    /// RadioTxPower property
    #[zbus(property)]
    fn radio_tx_power(&self) -> zbus::Result<u8>;

    /// RcpInterfaceMetrics property
    #[zbus(property)]
    fn rcp_interface_metrics(&self) -> zbus::Result<(u8, u64, u64, u64, u64, u64, u64, u64)>;

    /// Rloc16 property
    #[zbus(property)]
    fn rloc16(&self) -> zbus::Result<u16>;

    /// RouterID property
    #[zbus(property, name = "RouterID")]
    fn router_id(&self) -> zbus::Result<u8>;

    /// SrpServerInfo property
    #[zbus(property)]
    fn srp_server_info(
        &self,
    ) -> zbus::Result<(
        u8,
        u16,
        u8,
        (u32, u32, u64, u64, u64, u64),
        (u32, u32, u64, u64, u64, u64),
        (u32, u32, u32, u32, u32, u32),
    )>;

    /// StableNetworkData property
    #[zbus(property)]
    fn stable_network_data(&self) -> zbus::Result<Vec<u8>>;

    /// TelemetryData property
    #[zbus(property)]
    fn telemetry_data(&self) -> zbus::Result<Vec<u8>>;

    /// ThreadVersion property
    #[zbus(property)]
    fn thread_version(&self) -> zbus::Result<u16>;

    /// Uptime property
    #[zbus(property)]
    fn uptime(&self) -> zbus::Result<u64>;
}
