// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines, ReadHalf, WriteHalf};
use tokio::time::Duration;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use twine_codec::{
    Channel, ChannelMask, NetworkName, NetworkRole, OperationalDataset, PanId, Rloc16,
};

use crate::{error::TwineCtlError, TwineCtl};

use super::{SkipResultRead, TwineCtlShell};

pub struct TwineCtlSerialShell {
    prompt: Option<&'static str>,
    lines: Lines<BufReader<ReadHalf<SerialStream>>>,
    writer: WriteHalf<SerialStream>,
    timeout_duration: Duration,
}

impl TwineCtlSerialShell {
    /// Open a serial connection to a Thread device
    pub async fn open(path: &str, baud: u32) -> Result<Self, TwineCtlError> {
        let port = tokio_serial::new(path, baud).open_native_async()?;

        let (reader, writer) = tokio::io::split(port);
        let lines = BufReader::new(reader).lines();

        let mut shell = TwineCtlSerialShell {
            prompt: Some(">"),
            lines,
            writer,
            timeout_duration: Duration::from_millis(1_000),
        };

        shell.enter_and_wait_for_prompt().await?;

        Ok(shell)
    }

    /// Send enter key sequences and wait for the prompt
    async fn enter_and_wait_for_prompt(&mut self) -> Result<(), TwineCtlError> {
        self.writer.write_all(b"\r\n\r\n").await?;
        self.writer.flush().await?;
        self.wait_for_prompt(self.timeout_duration).await
    }
}

#[async_trait::async_trait]
impl TwineCtlShell for TwineCtlSerialShell {
    fn cmd_timeout_duration(&self) -> Duration {
        self.timeout_duration
    }

    fn prompt(&self) -> Option<&'static str> {
        self.prompt
    }

    async fn next_line(&mut self) -> Result<Option<String>, TwineCtlError> {
        let line = self.lines.next_line().await?;
        Ok(line)
    }

    async fn run(
        &mut self,
        cmd: &str,
        timeout_duration: Duration,
        skip_result_read: SkipResultRead,
    ) -> Result<Vec<String>, TwineCtlError> {
        self.writer.write_all(cmd.as_bytes()).await?;
        self.writer.write_all(b"\r\n").await?;
        self.writer.flush().await?;

        match skip_result_read {
            SkipResultRead::True => {
                self.enter_and_wait_for_prompt().await?;
                Ok(Vec::new())
            }
            SkipResultRead::False => self.read_result(cmd, timeout_duration).await,
        }
    }
}

#[async_trait::async_trait]
impl TwineCtl for TwineCtlSerialShell {
    async fn new_random_network(&mut self) -> Result<(), TwineCtlError> {
        self.shell_new_random_network().await
    }

    async fn active_dataset(&mut self) -> Result<OperationalDataset, TwineCtlError> {
        self.shell_active_dataset().await
    }

    async fn attach_with_dataset(
        &mut self,
        dataset: &OperationalDataset,
    ) -> Result<(), TwineCtlError> {
        self.shell_attach_with_dataset(dataset).await
    }

    async fn pending_dataset(&mut self) -> Result<OperationalDataset, TwineCtlError> {
        self.shell_pending_dataset().await
    }

    async fn channel(&mut self) -> Result<Channel, TwineCtlError> {
        self.shell_channel().await
    }

    async fn preferred_channel_mask(&mut self) -> Result<ChannelMask, TwineCtlError> {
        self.shell_preferred_channel_mask().await
    }

    async fn supported_channel_mask(&mut self) -> Result<ChannelMask, TwineCtlError> {
        self.shell_supported_channel_mask().await
    }

    async fn factory_reset(&mut self) -> Result<(), TwineCtlError> {
        self.shell_factory_reset().await
    }

    async fn network_name(&mut self) -> Result<NetworkName, TwineCtlError> {
        self.shell_network_name().await
    }

    async fn pan_id(&mut self) -> Result<PanId, TwineCtlError> {
        self.shell_pan_id().await
    }

    async fn reset(&mut self) -> Result<(), TwineCtlError> {
        self.shell_reset().await
    }

    async fn rloc16(&mut self) -> Result<Rloc16, TwineCtlError> {
        self.shell_rloc16().await
    }

    async fn role(&mut self) -> Result<NetworkRole, TwineCtlError> {
        self.shell_role().await
    }

    async fn version(&mut self) -> Result<String, TwineCtlError> {
        self.shell_version().await
    }

    async fn uptime(&mut self) -> Result<String, TwineCtlError> {
        self.shell_uptime().await
    }
}
