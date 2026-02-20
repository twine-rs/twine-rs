// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines, ReadHalf, WriteHalf};
use tokio::time::Duration;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use twine_rs_macros::TwineShell;

use crate::error::TwineCtlError;

use super::{SkipResultRead, TwineCtlShell};

#[derive(TwineShell)]
#[twine_shell(crate_path = "crate")]
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
                // Allow the device time to process the command and respond before attempting to
                // read the next prompt
                tokio::time::sleep(Duration::from_millis(5)).await;

                // Drain any stale boot messages or leftover output
                self.drain_lines(Duration::from_millis(5)).await?;

                // Establish a new prompt
                self.enter_and_wait_for_prompt().await?;

                // Drain any remaining data that arrived after the prompt
                self.drain_lines(Duration::from_millis(5)).await?;
                Ok(Vec::new())
            }
            SkipResultRead::False => self.read_result(cmd, timeout_duration).await,
        }
    }
}
