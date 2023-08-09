use crate::packets::{checksum::calculate_checksum, structures::EqualizerConfiguration};

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetEqualizerPacket {
    left_configuration: EqualizerConfiguration,
    right_configuration: Option<EqualizerConfiguration>,
}

impl SetEqualizerPacket {
    pub fn new(
        left_configuration: EqualizerConfiguration,
        right_configuration: Option<EqualizerConfiguration>,
    ) -> Self {
        Self {
            left_configuration,
            right_configuration,
        }
    }
}

impl OutboundPacket for SetEqualizerPacket {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0x08, 0xEE, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00];

        bytes.extend(self.left_configuration.profile_id().to_le_bytes());
        bytes.extend(self.left_configuration.volume_adjustments().bytes());
        bytes.extend(
            self.right_configuration
                .map(|right| right.volume_adjustments().bytes())
                .into_iter()
                .flatten(),
        );

        bytes.push(calculate_checksum(&bytes));

        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::{
        outbound::OutboundPacket,
        structures::{EqualizerConfiguration, PresetEqualizerProfile, VolumeAdjustments},
    };

    use super::SetEqualizerPacket;

    #[test]
    fn it_matches_an_example_custom_eq_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0xfe, 0xfe, 0x3c, 0xb4, 0x8f,
            0xa0, 0x8e, 0xb4, 0x74, 0x88, 0xe6,
        ];
        let packet = SetEqualizerPacket::new(
            EqualizerConfiguration::new_custom_profile(VolumeAdjustments::new([
                -60, 60, 23, 40, 22, 60, -4, 16,
            ])),
            None,
        );
        assert_eq!(EXPECTED, packet.bytes());
    }

    #[test]
    fn it_matches_an_example_soundcore_signature_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x00, 0x00, 0x78, 0x78, 0x78,
            0x78, 0x78, 0x78, 0x78, 0x78, 0x4d,
        ];
        let packet = SetEqualizerPacket::new(
            EqualizerConfiguration::new_from_preset_profile(
                PresetEqualizerProfile::SoundcoreSignature,
            ),
            None,
        );
        assert_eq!(EXPECTED, packet.bytes());
    }

    #[test]
    fn it_matches_an_example_treble_reducer_packet() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x15, 0x00, 0x78, 0x78, 0x78,
            0x64, 0x5a, 0x50, 0x50, 0x3c, 0xa4,
        ];
        let packet = SetEqualizerPacket::new(
            EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::TrebleReducer),
            None,
        );
        assert_eq!(EXPECTED, packet.bytes());
    }

    #[test]
    fn it_sends_second_channel_if_present() {
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x02, 0x81, 0x14, 0x00, 0x15, 0x00, 0x78, 0x78, 0x78,
            0x64, 0x5a, 0x50, 0x50, 0x3c, 0x78, 0x78, 0x78, 0x64, 0x5a, 0x50, 0x50, 0x3c, 0xa6,
        ];
        let configuration =
            EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::TrebleReducer);
        let packet = SetEqualizerPacket::new(configuration, Some(configuration));
        assert_eq!(EXPECTED, packet.bytes());
    }
}
