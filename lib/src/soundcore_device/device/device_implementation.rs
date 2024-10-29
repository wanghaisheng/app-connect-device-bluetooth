use std::collections::HashMap;

use crate::devices::standard::{
    state::DeviceState,
    structures::{
        AmbientSoundModeCycle, CustomButtonModel, EqualizerConfiguration, HearId, SoundModes,
        SoundModesTypeTwo,
    },
};

use super::soundcore_command::CommandResponse;

pub trait DeviceImplementation {
    fn packet_handlers(
        &self,
    ) -> HashMap<[u8; 7], Box<dyn Fn(&[u8], DeviceState) -> DeviceState + Send + Sync>>;

    fn initialize(&self, packet: &[u8]) -> crate::Result<DeviceState>;

    fn set_sound_modes(
        &self,
        state: DeviceState,
        sound_modes: SoundModes,
    ) -> crate::Result<CommandResponse>;

    fn set_sound_modes_type_two(
        &self,
        state: DeviceState,
        sound_modes: SoundModesTypeTwo,
    ) -> crate::Result<CommandResponse>;

    fn set_ambient_sound_mode_cycle(
        &self,
        state: DeviceState,
        cycle: AmbientSoundModeCycle,
    ) -> crate::Result<CommandResponse>;

    fn set_equalizer_configuration(
        &self,
        state: DeviceState,
        equalizer_configuration: EqualizerConfiguration,
    ) -> crate::Result<CommandResponse>;

    fn set_hear_id(&self, state: DeviceState, hear_id: HearId) -> crate::Result<CommandResponse>;

    fn set_custom_button_model(
        &self,
        state: DeviceState,
        custom_button_model: CustomButtonModel,
    ) -> crate::Result<CommandResponse>;
}
