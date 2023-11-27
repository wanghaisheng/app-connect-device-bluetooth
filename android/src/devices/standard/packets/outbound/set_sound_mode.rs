use openscq30_lib::devices::standard::packets::outbound::{
    OutboundPacketBytes, SetSoundModePacket as LibSetSoundModePacket,
};
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

use crate::{type_conversion, OutboundPacket, SoundModes};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetSoundModePacket {
    packet: LibSetSoundModePacket,
}

impl SetSoundModePacket {
    #[generate_interface(constructor)]
    pub fn new(sound_modes: &SoundModes) -> SetSoundModePacket {
        Self {
            packet: LibSetSoundModePacket {
                ambient_sound_mode: sound_modes.ambient_sound_mode().into(),
                noise_canceling_mode: sound_modes.noise_canceling_mode().into(),
                transparency_mode: sound_modes.transparency_mode().into(),
                custom_noise_canceling: sound_modes.custom_noise_canceling().into(),
            },
        }
    }
}

impl OutboundPacket for SetSoundModePacket {
    #[generate_interface]
    fn bytes(&self) -> Vec<i8> {
        type_conversion::u8_slice_to_i8_slice(&self.packet.bytes()).to_vec()
    }
}