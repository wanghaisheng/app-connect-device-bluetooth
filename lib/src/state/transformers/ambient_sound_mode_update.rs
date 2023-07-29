use crate::{
    packets::{inbound::SoundModeUpdatePacket, structures::SoundModes},
    state::{DeviceState, DeviceStateTransformer},
};

impl DeviceStateTransformer for SoundModeUpdatePacket {
    fn transform(&self, state: &DeviceState) -> DeviceState {
        DeviceState {
            sound_modes: Some(SoundModes {
                ambient_sound_mode: self.ambient_sound_mode,
                noise_canceling_mode: self.noise_canceling_mode,
                custom_noise_canceling: self.custom_noise_canceling,
                transparency_mode: self.transparency_mode,
            }),
            ..state.clone()
        }
    }
}
