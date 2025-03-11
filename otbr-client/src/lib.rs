use std::time::Duration;

use async_trait::async_trait;
use zbus::Result;

use twine_codec::{
    commissioner::JoinerPskd,
    dataset::{
        ExtendedPanId, NetworkKey, NetworkName, OperationalDataset, OperationalDatasetTlvs, Pskc,
    },
    link::{ActiveScanResult, EnergyScanResult},
    radio::{Channel, ChannelMask, PanId},
    thread::DeviceRole,
};

use dbus::{BorderRouterProxy, OtbrDbusClient};
use error::OtbrClientError;

pub mod dbus;
pub mod error;

pub type OtbrClientResult<T> = std::result::Result<T, OtbrClientError>;

#[async_trait]
pub trait OtbrClient {
    async fn active_dataset_tlvs(&self) -> OtbrClientResult<OperationalDatasetTlvs>;

    /// Attach the current device to the Thread network.
    async fn attach(
        &self,
        key: Option<NetworkKey>,
        pan: Option<PanId>,
        name: NetworkName,
        xpan: Option<ExtendedPanId>,
        pskc: Option<Pskc>,
        channel_mask: ChannelMask,
    ) -> OtbrClientResult<()>;

    /// Request all available nodes attach to the specified Thread network.
    ///
    /// Returns the time it will take to attach all nodes to the network. If the
    /// value is `None`, the network attachment was immediate.
    async fn attach_all_nodes_to(
        &self,
        dataset: OperationalDatasetTlvs,
    ) -> OtbrClientResult<Option<Duration>>;

    /// Detach the current device from the Thread network.
    async fn detach(&self) -> OtbrClientResult<()>;

    /// The current device role.
    async fn device_role(&self) -> OtbrClientResult<DeviceRole>;

    /// Perform an IEEE 802.15.4 energy scan.
    ///
    /// * `duration` - Duration of time for the scan of each channel.
    async fn energy_scan(&self, duration: Duration) -> OtbrClientResult<Vec<EnergyScanResult>>;

    /// Perform a factory reset and wipe all Thread persistent data.
    async fn factory_reset(&self) -> OtbrClientResult<()>;

    // /// Start Thread joining.
    // async fn joiner_start(
    //     &self,
    //     pskd: JoinerPskd,
    //     url: ProvisioningUrl,
    //     vendor_info: ThreadVendorInfo,
    // ) -> OtbrClientResult<()>;

    /// Stop Thread joining.
    async fn joiner_stop(&self) -> OtbrClientResult<()>;

    /// Detach from the network and forget the network credentials.
    async fn leave_network(&self) -> OtbrClientResult<()>;

    /// Allow joining the network via unsecure traffic temporarily.
    async fn permit_unsecure_join(&self, port: u16, duration: Duration) -> OtbrClientResult<()>;

    /// Perform a reset and attempt to resume the network after the reset.
    async fn reset(&self) -> OtbrClientResult<()>;

    /// Perform a Thread network scan.
    async fn scan(&self) -> OtbrClientResult<Vec<ActiveScanResult>>;
}
