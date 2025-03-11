use std::time::Duration;

use async_trait::async_trait;
use zbus::{proxy, Connection, Result};

use twine_codec::{
    commissioner::{JoinerPskd, ProvisioningUrl, ThreadVendorInfo},
    dataset::{ChannelMask, DatasetTlvs, ExtendedPanId, NetworkKey, NetworkName, PanId, Pskc},
    link::{ActiveScanResult, EnergyScanResult},
};

pub use dbus::{BorderRouterProxy, OtbrDbusClient};

mod dbus;

#[async_trait]
pub trait OtbrClient {
    /// Attach the current device to the Thread network.
    async fn attach(
        &self,
        key: Option<NetworkKey>,
        pan: Option<PanId>,
        name: NetworkName,
        xpan: Option<ExtendedPanId>,
        pskc: Option<Pskc>,
        channel_mask: Option<ChannelMask>,
    ) -> Result<()>;

    /// Request all available nodes attach to the specified Thread network.
    ///
    /// Returns the time it will take to attach all nodes to the network. If the
    /// value is `None`, the network attachment was immediate.
    async fn attach_all_nodes_to(&self, dataset: DatasetTlvs) -> Result<Option<Duration>>;

    /// Detach the current device from the Thread network.
    async fn detach(&self) -> Result<()>;

    /// Perform an IEEE 802.15.4 energy scan.
    async fn energy_scan(&self, duration: Duration) -> Result<Vec<EnergyScanResult>>;

    /// Perform a factory reset and wipe all Thread persistent data.
    async fn factory_reset(&self) -> Result<()>;

    /// Start Thread joining.
    async fn joiner_start(
        &self,
        pskd: JoinerPskd,
        url: ProvisioningUrl,
        vendor_info: ThreadVendorInfo,
    ) -> Result<()>;

    /// Stop Thread joining.
    async fn joiner_stop(&self) -> Result<()>;

    /// Detach from the network and forget the network credentials.
    async fn leave_network(&self) -> Result<()>;

    /// Allow joining the network via unsecure traffic temporarily.
    async fn permit_unsecure_join(&self, port: u16, duration: Duration) -> Result<()>;

    /// Perform a reset and attempt to resume the network after the reset.
    async fn reset(&self) -> Result<()>;

    /// Perform a Thread network scan.
    async fn scan(&self) -> Result<Vec<ActiveScanResult>>;
}
