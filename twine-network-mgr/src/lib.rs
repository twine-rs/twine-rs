use std::path::PathBuf;

use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;
use otbr_client::OtbrClient;
use thiserror::Error;
use typed_builder::TypedBuilder;

use twine_codec::{dataset::OperationalDatasetTlvs, error::TwineCodecError};

#[derive(Message)]
#[rtype(result = "Response")]
pub enum Request {
    VersionInfo,
}

#[derive(Debug)]
pub enum Response {
    VersionInfo(String),
}

impl<A, M> MessageResponse<A, M> for Response
where
    A: Actor,
    M: Message<Result = Response>,
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

#[derive(Debug, Error)]
pub enum TwineNetworkMgrError {}

#[derive(Default)]
pub struct ThreadNetworkSettings {
    dataset: OperationalDatasetTlvs,
    nat64_enabled: bool,
}

#[derive(Default)]
pub enum ThreadNetworkState {
    /// The Thread network state is unknown.
    #[default]
    Unknown,
    /// The Thread network is disabled.
    ///
    /// In this state, the management state machine will not attempt to start the network.
    Disabled,
    /// The Thread network is down.
    ///
    /// This state occurs when the Thread network is expected to be enabled, but is not running or
    /// communication with the Thread radio is not currently possible. The state machine will
    /// transition to `Up` when communication is restored.
    Down,
    /// The Thread network is up, but not yet configured.
    ///
    /// The state machine will remain in this state until all information required to configure the
    /// network is available and `start` is called.
    Up,
    /// The Thread network is running and the operational dataset has been configured.
    ///
    /// Operational dataset changes may continue to occur in this state.
    UpAndRunning,
}

#[derive(TypedBuilder)]
pub struct ThreadNetworkMgr {
    state: ThreadNetworkState,
    mem_cache_settings: ThreadNetworkSettings,
    persisted_settings: Option<PathBuf>,
    otbr_client: Box<dyn OtbrClient>,
}

impl Handler<Request> for ThreadNetworkMgr {
    type Result = Response;

    fn handle(&mut self, _msg: Request, _ctx: &mut Context<Self>) -> Self::Result {
        match _msg {
            Request::VersionInfo => Response::VersionInfo("0.1.0".to_string()),
        }
    }
}

impl Actor for ThreadNetworkMgr {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("ThreadNetworkMgr started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("ThreadNetworkMgr stopped");
    }
}
