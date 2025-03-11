use bitflags::bitflags;

bitflags! {
    /// IEEE 802.15.4 channel page mask
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct ChannelPageMask: u8 {
        /// 2.4 GHz O-QPSK PHY
        const PAGE_0 = 0;
    }

    /// IEEE 802.15.4 Channel Mask
    ///
    /// Each bit in the channel mask represents the selected channel.
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct ChannelMaskBits: u32 {
        const CHANNEL_0 = 1 << 0;
        const CHANNEL_1 = 1 << 1;
        const CHANNEL_2 = 1 << 2;
        const CHANNEL_3 = 1 << 3;
        const CHANNEL_4 = 1 << 4;
        const CHANNEL_5 = 1 << 5;
        const CHANNEL_6 = 1 << 6;
        const CHANNEL_7 = 1 << 7;
        const CHANNEL_8 = 1 << 8;
        const CHANNEL_9 = 1 << 9;
        const CHANNEL_10 = 1 << 10;
        const CHANNEL_11 = 1 << 11;
        const CHANNEL_12 = 1 << 12;
        const CHANNEL_13 = 1 << 13;
        const CHANNEL_14 = 1 << 14;
        const CHANNEL_15 = 1 << 15;
        const CHANNEL_16 = 1 << 16;
        const CHANNEL_17 = 1 << 17;
        const CHANNEL_18 = 1 << 18;
        const CHANNEL_19 = 1 << 19;
        const CHANNEL_20 = 1 << 20;
        const CHANNEL_21 = 1 << 21;
        const CHANNEL_22 = 1 << 22;
        const CHANNEL_23 = 1 << 23;
        const CHANNEL_24 = 1 << 24;
        const CHANNEL_25 = 1 << 25;
        const CHANNEL_26 = 1 << 26;
        const CHANNEL_27 = 1 << 27;
        const CHANNEL_28 = 1 << 28;
        const CHANNEL_29 = 1 << 29;
        const CHANNEL_30 = 1 << 30;
        const CHANNEL_31 = 1 << 31;
    }
}

impl From<ChannelPageMask> for u8 {
    fn from(value: ChannelPageMask) -> Self {
        value.bits()
    }
}

impl From<u8> for ChannelPageMask {
    fn from(value: u8) -> Self {
        match value {
            0 => ChannelPageMask::PAGE_0,
            _ => panic!("Invalid channel page mask: {}", value),
        }
    }
}

pub struct ChannelMask {
    page: ChannelPageMask,
    /// Number of octets in the channel mask
    len: u8,
    mask: ChannelMaskBits,
}

impl ChannelMask {
    pub fn mask(&self) -> u32 {
        self.mask.bits()
    }

    /// Reverse the bits of the channel mask to match the TLV format listed in the Thread 1.4.0 specification.
    ///
    /// Thread 1.4.0 8.10.1.18.1 Channel Mask Entry
    pub fn to_tlv_entry(self) -> (u8, u8, u32) {
        (self.page.into(), self.len, self.mask.bits().reverse_bits())
    }

    pub fn from_tlv_entry(value: (u8, u32)) -> Self {
        let page = value.0;
        let len = 4;
        let mask = ChannelMaskBits::from_bits_truncate(value.1.reverse_bits());
        ChannelMask {
            page: page.into(),
            len,
            mask,
        }
    }
}

/// IEEE 802.15.4 channel
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Channel {
    channel: u16,
    page: u8,
}

impl Channel {
    pub fn new(channel: u16, page: u8) -> Self {
        Self { channel, page }
    }

    pub fn channel(&self) -> u16 {
        self.channel
    }

    pub fn page(&self) -> u8 {
        self.page
    }
}
