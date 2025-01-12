use nom::{
    combinator::{all_consuming, opt},
    error::{context, ContextError, ParseError},
    number::complete::{le_u16, le_u8},
    sequence::tuple,
};

use crate::devices::{
    a3951::device_profile::A3951_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::{state_update_packet::StateUpdatePacket, InboundPacket},
            parsing::{take_bool, ParseResult},
        },
        structures::{
            AgeRange, CustomButtonModel, CustomHearId, DualBattery, EqualizerConfiguration, Gender,
            HostDevice, SoundModes, StereoEqualizerConfiguration,
        },
    },
};

// A3951
#[derive(Debug, Clone, PartialEq)]
pub struct A3951StateUpdatePacket {
    host_device: HostDevice,
    tws_status: bool,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration,
    gender: Gender,
    age_range: AgeRange,
    custom_hear_id: CustomHearId,
    custom_button_model: CustomButtonModel,
    sound_modes: SoundModes,
    side_tone: bool,
    wear_detection: bool,
    touch_tone: bool,
    hear_id_eq_preset: Option<u16>,
    supports_new_battery: bool, // yes if packet is >98, don't parse
    left_new_battery: u8,       // 0 to 9
    right_new_battery: u8,      // 0 to 9
}

impl From<A3951StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3951StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3951_DEVICE_PROFILE,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: Some(packet.age_range),
            gender: Some(packet.gender),
            hear_id: Some(packet.custom_hear_id.into()),
            custom_button_model: Some(packet.custom_button_model),
            firmware_version: None,
            serial_number: None,
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
}

impl InboundPacket for A3951StateUpdatePacket {
    fn command() -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<A3951StateUpdatePacket, E> {
        context(
            "a3951 state update packet",
            all_consuming(|input| {
                // required fields
                let (
                    input,
                    (
                        host_device,
                        tws_status,
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        custom_hear_id,
                        custom_button_model,
                        sound_modes,
                        side_tone,
                        wear_detection,
                        touch_tone,
                    ),
                ) = tuple((
                    HostDevice::take,
                    take_bool, // tws status
                    DualBattery::take,
                    StereoEqualizerConfiguration::take(8),
                    Gender::take,
                    AgeRange::take,
                    CustomHearId::take_with_all_fields,
                    CustomButtonModel::take,
                    SoundModes::take,
                    take_bool, // side tone
                    take_bool, // wear detection
                    take_bool, // touch tone
                ))(input)?;

                // >=96 length optional fields
                let (input, hear_id_eq_preset) = opt(le_u16)(input)?;

                // >=98 length optional fields
                let (input, new_battery) = opt(tuple((le_u8, le_u8)))(input)?;

                Ok((
                    input,
                    A3951StateUpdatePacket {
                        host_device,
                        tws_status,
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        custom_hear_id,
                        custom_button_model,
                        sound_modes,
                        side_tone,
                        wear_detection,
                        touch_tone,
                        hear_id_eq_preset,
                        supports_new_battery: new_battery.is_some(),
                        left_new_battery: new_battery.as_ref().map(|b| b.0).unwrap_or_default(),
                        right_new_battery: new_battery.as_ref().map(|b| b.1).unwrap_or_default(),
                    },
                ))
            }),
        )(input)
    }
}
