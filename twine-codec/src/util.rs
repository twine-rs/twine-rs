// Copyright (c) 2026 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Fill a buffer with random bytes
///
/// Warning: This function is not meant for cryptographic operations.
pub(crate) fn fill_random_bytes(buffer: &mut [u8]) {
    getrandom::fill(buffer).expect("failed to fill buffer with random bytes");
}

/// Generate a random u16 value in the given inclusive range
///
/// Warning: This function is not meant for cryptographic operations.
pub(crate) fn random_range_u16(range: core::ops::RangeInclusive<u16>) -> u16 {
    let (min, max) = (*range.start(), *range.end());
    let mut bytes = [0u8; 2];
    fill_random_bytes(&mut bytes);
    let random = u16::from_le_bytes(bytes);
    let span = (max - min) as u32 + 1;
    min + ((random as u32 * span) >> 16) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_random_bytes_fills_buffer() {
        let mut buffer = [0u8; 16];
        fill_random_bytes(&mut buffer);
        // Extremely unlikely that all bytes remain zero after random fill
        assert!(buffer.iter().any(|&b| b != 0));
    }

    #[test]
    fn fill_random_bytes_produces_different_values() {
        let mut buffer1 = [0u8; 16];
        let mut buffer2 = [0u8; 16];
        fill_random_bytes(&mut buffer1);
        fill_random_bytes(&mut buffer2);
        // Extremely unlikely that two random fills produce identical results
        assert_ne!(buffer1, buffer2);
    }

    #[test]
    fn random_range_u16_within_bounds() {
        for _ in 0..1000 {
            let value = random_range_u16(10..=20);
            assert!(
                value >= 10 && value <= 20,
                "value {} out of range 10..=20",
                value
            );
        }
    }

    #[test]
    fn random_range_u16_single_value() {
        for _ in 0..100 {
            let value = random_range_u16(42..=42);
            assert_eq!(value, 42);
        }
    }

    #[test]
    fn random_range_u16_full_range() {
        // Test with a wider range to ensure distribution works
        let mut seen_low = false;
        let mut seen_high = false;
        for _ in 0..1000 {
            let value = random_range_u16(0..=100);
            if value < 20 {
                seen_low = true;
            }
            if value > 80 {
                seen_high = true;
            }
        }
        assert!(seen_low, "never saw values in low range");
        assert!(seen_high, "never saw values in high range");
    }

    #[test]
    fn random_range_u16_edge_values() {
        for _ in 0..1000 {
            let value = random_range_u16(0..=u16::MAX);
            // Just verify it doesn't panic and returns a valid u16
            let _ = value;
        }
    }
}
