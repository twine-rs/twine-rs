use actix::prelude::*;

use twine_network_mgr::{Request, ThreadNetworkMgr, ThreadNetworkSettings, ThreadNetworkState};

#[actix::main]
async fn main() {
    let otbr_client = otbr_client::dbus::OtbrDbusClient::new().await.unwrap();

    let net_mgr = ThreadNetworkMgr::builder()
        .mem_cache_settings(ThreadNetworkSettings::default())
        .persisted_settings(None)
        .otbr_client(Box::new(otbr_client))
        .state(ThreadNetworkState::default())
        .build();

    let addr = net_mgr.start();

    let result = addr.send(Request::VersionInfo).await;
    println!("Version: {:?}", result);
}
