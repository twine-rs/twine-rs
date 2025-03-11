use core::time::Duration;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Authoritative(bool);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Timestamp {
    seconds: u64,
    ticks: u16,
    auth: Authoritative,
}

impl Timestamp {
    #[cfg(feature = "std")]
    pub fn now(auth: Authoritative) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now();
        Self {
            seconds: now.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            ticks: 0,
            auth,
        }
    }

    pub fn since_epoch(&self) -> Duration {
        // Duration::from_secs(self.seconds)
        todo!()
    }
}

impl From<u64> for Timestamp {
    fn from(value: u64) -> Self {
        todo!()
    }
}

impl From<Timestamp> for u64 {
    fn from(timestamp: Timestamp) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn since_epoch() {
        // todo!()
    }
}
