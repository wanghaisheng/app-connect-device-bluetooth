use crate::packets::structures::equalizer_configuration::EqualizerConfiguration;

use super::{outbound_packet::OutboundPacket, utils};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetEqualizerPacket {
    configuration: EqualizerConfiguration,
}

impl SetEqualizerPacket {
    pub fn new(configuration: EqualizerConfiguration) -> Self {
        Self { configuration }
    }
}

impl OutboundPacket for SetEqualizerPacket {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0x08, 0xEE, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00];

        bytes.extend(self.configuration.profile_id().to_le_bytes());
        bytes.extend(self.configuration.band_offsets().bytes());

        bytes.push(utils::calculate_checksum(&bytes));

        bytes
    }
}
