// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod error;
mod shell;

pub use error::TwineCtlError;
pub use shell::TwineCtlSerialShell;

use twine_codec::{
    Channel, ChannelMask, NetworkName, NetworkRole, OperationalDataset, PanId, Rloc16,
};

/// Control interface for Thread devices
#[async_trait::async_trait]
pub trait TwineCtl {
    async fn new_random_network(&mut self) -> Result<(), TwineCtlError>;
    async fn active_dataset(&mut self) -> Result<OperationalDataset, TwineCtlError>;
    async fn attach_with_dataset(
        &mut self,
        dataset: &OperationalDataset,
    ) -> Result<(), TwineCtlError>;
    async fn pending_dataset(&mut self) -> Result<OperationalDataset, TwineCtlError>;
    async fn channel(&mut self) -> Result<Channel, TwineCtlError>;
    async fn preferred_channel_mask(&mut self) -> Result<ChannelMask, TwineCtlError>;
    async fn supported_channel_mask(&mut self) -> Result<ChannelMask, TwineCtlError>;
    async fn factory_reset(&mut self) -> Result<(), TwineCtlError>;
    async fn network_name(&mut self) -> Result<NetworkName, TwineCtlError>;
    async fn pan_id(&mut self) -> Result<PanId, TwineCtlError>;
    async fn reset(&mut self) -> Result<(), TwineCtlError>;
    async fn rloc16(&mut self) -> Result<Rloc16, TwineCtlError>;
    async fn role(&mut self) -> Result<NetworkRole, TwineCtlError>;
    async fn version(&mut self) -> Result<String, TwineCtlError>;
    async fn uptime(&mut self) -> Result<String, TwineCtlError>;
}
