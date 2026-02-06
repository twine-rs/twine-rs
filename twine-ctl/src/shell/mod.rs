// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{str::FromStr, time::Duration};

use twine_codec::{
    Channel, ChannelMask, NetworkName, NetworkRole, OperationalDataset, PanId, Rloc16,
};

use crate::TwineCtlError;

pub mod serial;

pub use serial::TwineCtlSerialShell;

enum SkipResultRead {
    True,
    False,
}

#[async_trait::async_trait]
trait TwineCtlShell {
    /// Get the timeout duration for shell commands
    fn cmd_timeout_duration(&self) -> Duration;

    /// Get the prompt indicator string, if any
    fn prompt(&self) -> Option<&str>;

    /// Get the next line for processing from the shell
    async fn next_line(&mut self) -> Result<Option<String>, TwineCtlError>;

    /// Run a command and return the resulting output lines
    async fn run(
        &mut self,
        cmd: &str,
        timeout_duration: Duration,
        skip_result_read: SkipResultRead,
    ) -> Result<Vec<String>, TwineCtlError>;

    /// Read result lines until "Done" or "Error" is encountered
    async fn read_result(
        &mut self,
        cmd: &str,
        timeout_duration: Duration,
    ) -> Result<Vec<String>, TwineCtlError> {
        let mut out: Vec<String> = Vec::new();
        let read_fut = async {
            log::trace!("Sending command: {}", cmd);
            loop {
                let maybe_line = self.next_line().await?;
                let Some(line) = maybe_line else {
                    return Err(TwineCtlError::UnexpectedEof);
                };

                let line = line.trim();
                log::trace!("Line: {}", line);

                // skip empty line
                if line.is_empty() {
                    continue;
                }

                // skip echo
                if line.contains(cmd) {
                    continue;
                }

                // skip prompt
                if let Some(prompt) = self.prompt() {
                    if line.contains(prompt) {
                        continue;
                    }
                }

                if line == "Done" {
                    return Ok(());
                }

                if line == "Error" || line.starts_with("Error ") {
                    return Err(TwineCtlError::CommandError(line.to_string()));
                }

                out.push(line.to_string());
            }
        };

        match tokio::time::timeout(timeout_duration, read_fut).await {
            Ok(Ok(())) => Ok(out),
            Ok(Err(e)) => {
                log::error!("Error reading shell result: {:?}", e);
                Err(e)
            }
            Err(_) => {
                log::error!("Timeout reading shell result for command: {}", cmd);
                Err(TwineCtlError::TimeoutError)
            }
        }
    }

    async fn wait_for_prompt(&mut self, timeout_duration: Duration) -> Result<(), TwineCtlError> {
        let read_fut = async {
            loop {
                let maybe_line = self.next_line().await?;
                let Some(line) = maybe_line else {
                    return Err(TwineCtlError::UnexpectedEof);
                };

                let line = line.trim();
                log::trace!("Line: {}", line);

                if let Some(prompt) = self.prompt() {
                    if line.contains(prompt) {
                        return Ok(());
                    }
                }
            }
        };

        match tokio::time::timeout(timeout_duration, read_fut).await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => {
                log::error!("Error reading shell prompt: {:?}", e);
                Err(e)
            }
            Err(_) => {
                log::error!("Timeout reading shell prompt");
                Err(TwineCtlError::TimeoutError)
            }
        }
    }

    async fn shell_new_random_network(&mut self) -> Result<(), TwineCtlError> {
        self.run(
            "dataset init new",
            self.cmd_timeout_duration(),
            SkipResultRead::False,
        )
        .await?;
        self.run(
            "dataset commit active",
            self.cmd_timeout_duration(),
            SkipResultRead::False,
        )
        .await?;
        self.run(
            "ifconfig up",
            self.cmd_timeout_duration(),
            SkipResultRead::False,
        )
        .await?;
        self.run(
            "thread start",
            self.cmd_timeout_duration(),
            SkipResultRead::False,
        )
        .await?;
        Ok(())
    }

    async fn shell_active_dataset(&mut self) -> Result<OperationalDataset, TwineCtlError> {
        let lines = self
            .run(
                "dataset active -x",
                self.cmd_timeout_duration(),
                SkipResultRead::False,
            )
            .await?;

        let dataset_str = lines
            .into_iter()
            .next()
            .ok_or(TwineCtlError::ShellParseError)?;

        Ok(OperationalDataset::from_str(&dataset_str)?)
    }

    async fn shell_attach_with_dataset(
        &mut self,
        dataset: &OperationalDataset,
    ) -> Result<(), TwineCtlError> {
        let dataset_hex = dataset.as_hex_string();
        self.run(
            &format!("dataset init tlvs {}", dataset_hex),
            self.cmd_timeout_duration(),
            SkipResultRead::False,
        )
        .await?;

        self.run(
            "dataset commit active",
            self.cmd_timeout_duration(),
            SkipResultRead::False,
        )
        .await?;

        self.run(
            "ifconfig up",
            self.cmd_timeout_duration(),
            SkipResultRead::False,
        )
        .await?;

        self.run(
            "thread start",
            self.cmd_timeout_duration(),
            SkipResultRead::False,
        )
        .await?;

        Ok(())
    }

    async fn shell_pending_dataset(&mut self) -> Result<OperationalDataset, TwineCtlError> {
        let lines = self
            .run(
                "dataset pending -x",
                self.cmd_timeout_duration(),
                SkipResultRead::False,
            )
            .await?;

        let dataset_str = lines
            .into_iter()
            .next()
            .ok_or(TwineCtlError::ShellParseError)?;

        Ok(OperationalDataset::from_str(&dataset_str)?)
    }

    async fn shell_channel(&mut self) -> Result<Channel, TwineCtlError> {
        let lines = self
            .run(
                "channel",
                self.cmd_timeout_duration(),
                SkipResultRead::False,
            )
            .await?;

        let channel_str = lines
            .into_iter()
            .next()
            .ok_or(TwineCtlError::ShellParseError)?;

        let channel = Channel::from_str_channel_only(&channel_str)?;

        Ok(channel)
    }

    async fn shell_preferred_channel_mask(&mut self) -> Result<ChannelMask, TwineCtlError> {
        let lines = self
            .run(
                "channel preferred",
                self.cmd_timeout_duration(),
                SkipResultRead::False,
            )
            .await?;

        let channel_mask_str = lines
            .into_iter()
            .next()
            .ok_or(TwineCtlError::ShellParseError)?;

        let channel_mask = ChannelMask::from_str(&channel_mask_str)?;

        Ok(channel_mask)
    }

    async fn shell_supported_channel_mask(&mut self) -> Result<ChannelMask, TwineCtlError> {
        let lines = self
            .run(
                "channel supported",
                self.cmd_timeout_duration(),
                SkipResultRead::False,
            )
            .await?;

        let channel_mask_str = lines
            .into_iter()
            .next()
            .ok_or(TwineCtlError::ShellParseError)?;

        let channel_mask = ChannelMask::from_str(&channel_mask_str)?;

        Ok(channel_mask)
    }

    async fn shell_factory_reset(&mut self) -> Result<(), TwineCtlError> {
        self.run(
            "factoryreset",
            self.cmd_timeout_duration(),
            SkipResultRead::True,
        )
        .await?;

        Ok(())
    }

    async fn shell_network_name(&mut self) -> Result<NetworkName, TwineCtlError> {
        let lines = self
            .run(
                "networkname",
                self.cmd_timeout_duration(),
                SkipResultRead::False,
            )
            .await?;

        let name_str = lines
            .into_iter()
            .next()
            .ok_or(TwineCtlError::ShellParseError)?;

        Ok(NetworkName::from_str(&name_str)?)
    }

    async fn shell_pan_id(&mut self) -> Result<twine_codec::PanId, TwineCtlError> {
        let lines = self
            .run("panid", self.cmd_timeout_duration(), SkipResultRead::False)
            .await?;

        let pan_id_str = lines
            .into_iter()
            .next()
            .ok_or(TwineCtlError::ShellParseError)?;

        Ok(PanId::from_str(&pan_id_str)?)
    }

    async fn shell_reset(&mut self) -> Result<(), TwineCtlError> {
        self.run("reset", self.cmd_timeout_duration(), SkipResultRead::True)
            .await?;
        Ok(())
    }

    async fn shell_rloc16(&mut self) -> Result<Rloc16, TwineCtlError> {
        let lines = self
            .run("rloc16", self.cmd_timeout_duration(), SkipResultRead::False)
            .await?;

        let rloc16_str = lines
            .into_iter()
            .next()
            .ok_or(TwineCtlError::ShellParseError)?;

        let rloc16 = Rloc16::from_str(&rloc16_str)?;

        Ok(rloc16)
    }

    async fn shell_role(&mut self) -> Result<twine_codec::NetworkRole, TwineCtlError> {
        let lines = self
            .run("state", self.cmd_timeout_duration(), SkipResultRead::False)
            .await?;

        let role_str = lines
            .into_iter()
            .next()
            .ok_or(TwineCtlError::ShellParseError)?;

        Ok(NetworkRole::from_str(&role_str)?)
    }

    async fn shell_version(&mut self) -> Result<String, TwineCtlError> {
        let lines = self
            .run(
                "version",
                self.cmd_timeout_duration(),
                SkipResultRead::False,
            )
            .await?;

        lines
            .into_iter()
            .next()
            .ok_or(TwineCtlError::ShellParseError)
    }

    async fn shell_uptime(&mut self) -> Result<String, TwineCtlError> {
        let lines = self
            .run("uptime", self.cmd_timeout_duration(), SkipResultRead::False)
            .await?;

        lines
            .into_iter()
            .next()
            .ok_or(TwineCtlError::ShellParseError)
    }
}
