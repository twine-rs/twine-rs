use core::str::FromStr;

use crate::error::TwineCodecError;

const NETWORK_NAME_MAX_SIZE: usize = 16;

/// A Thread Network Name
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkName([u8; NETWORK_NAME_MAX_SIZE + 1]);

impl NetworkName {
    fn type_name() -> &'static str {
        "NetworkName"
    }
}

impl core::fmt::Display for NetworkName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut length = 0;

        for byte in self.0.iter() {
            if *byte == 0 {
                break;
            }
            length += 1;
        }

        let s = core::str::from_utf8(&self.0[..length]).map_err(|_| core::fmt::Error)?;
        write!(f, "{}", s)
    }
}

impl FromStr for NetworkName {
    type Err = TwineCodecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.as_bytes();
        let length = raw.len();

        if length > NETWORK_NAME_MAX_SIZE {
            return Err(TwineCodecError::BufferMaxLength(
                Self::type_name(),
                NETWORK_NAME_MAX_SIZE,
                length,
            ));
        }

        let mut n = [0_u8; NETWORK_NAME_MAX_SIZE + 1];
        raw.iter().enumerate().for_each(|(i, byte)| {
            n[i] = *byte;
        });

        Ok(Self(n))
    }
}
